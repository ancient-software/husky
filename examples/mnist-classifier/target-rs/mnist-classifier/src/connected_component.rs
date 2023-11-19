struct ConnectedComponentDistribution{
    row_start: i32,
    row_end: i32,
    upper_mass: i32,
    lower_mass: i32,
}

struct EffHoles{
    matches: Vec<Option<Leash<RawContour>>>,
}

pub fn hole_tmpl(ct: Leash<RawContour>) -> Option<f32> {
    let len = ct.contour_len;
    require!(len > 4);
    len + 0
}

struct ConnectedComponent{
    mask: BinaryImage28,
}

pub fn horizontal_extend(a: r32, x: r32) -> r32 {
    let mut y = a | (x | x << 1 | x >> 1);
    let mut z = a | (y | y << 1 | y >> 1);
    while z != y {
        y = z;
        z = a | (y | y << 1 | y >> 1)
    }
    return y;
}

pub fn find_connected_components(img: BinaryImage28) -> Vec<ConnectedComponent> {
    let mut result: Vec<ConnectedComponent> = vec![];
    let mut unsearched = img.clone();
    for j in 0..30 {
        while unsearched[j] {
            let a = unsearched[j];
            let shift = a.ctz();
            let mut mask = BinaryImage28::new_zeros();
            mask[j] = horizontal_extend(a, 1 << shift);
            let mut flag = false;
            while !flag {
                flag = true;
                let mut i = j;
                while i < 30 - 1 {
                    {
                        let old_row = mask[i + 1];
                        let new_row = old_row | horizontal_extend(img[i + 1], mask[i]);
                        if !new_row {
                            break;
                        }
                        if old_row != new_row {
                            flag = false;
                            mask[i + 1] = new_row
                        }
                    }
                    i+= 1
                }
                while i >= j {
                    {
                        let old_row = mask[i];
                        let new_row = old_row | horizontal_extend(img[i], mask[i + 1]);
                        if old_row != new_row {
                            flag = false;
                            mask[i] = new_row
                        }
                    }
                    i-= 1
                }
            }
            for k in j..30 {
                unsearched[k] &= !mask[k]
            }
            result.push(ConnectedComponent(mask))
        }
    }
    return result;
}

impl Visualize for ConnectedComponent {
    fn visualize(self) {
        self.mask.visualize()
    }
}

impl ConnectedComponent {
    fn raw_contours(self) {
        find_raw_contours(self)
    }

    fn eff_holes(self) {
        let mut raw_contours = self.raw_contours.collect_leashes();
        let mut matches: Vec<Option<Leash<RawContour>>> = vec![];
        raw_contours.pop_with_largest_opt_f32(hole_tmpl);
        matches.push(raw_contours.pop_with_largest_opt_f32(hole_tmpl));
        matches.push(raw_contours.pop_with_largest_opt_f32(hole_tmpl));
        return EffHoles(matches);
    }

    fn max_hole_ilen(self) {
        let mut max_hole_ilen = 0;
        let raw_contours = self.raw_contours;
        for i in (0 + 1)..raw_contours.ilen() {
            let hole_ilen = raw_contours[i].points.ilen();
            if max_hole_ilen < hole_ilen {
                max_hole_ilen = hole_ilen
            }
        }
        return max_hole_ilen as f32;
    }

    fn max_row_span(self) {
        let mut max_row: i32 = 0;
        for i in (0 + 1)..29 {
            max_row = max_row.max(self.mask[i].span())
        }
        return max_row as f32;
    }

    fn row_span_sum(self) {
        let mut row_span_sum = 0;
        for i in (0 + 1)..29 {
            row_span_sum += self.mask[i].span()
        }
        return row_span_sum as f32;
    }

    fn distribution(self) {
        let mut row_start = 1;
        while row_start < 29 {
            {
                if self.mask[row_start] {
                    break;
                }
            }
            row_start+= 1
        }
        let mut row_end = row_start + 1;
        while row_end < 29 {
            {
                if !self.mask[row_end] {
                    break;
                }
            }
            row_end+= 1
        }
        let height = row_end - row_start;
        let half_height = height / 2;
        let mut upper_mass = 0;
        for i1 in row_start..row_start + half_height {
            upper_mass += self.mask[i1].co()
        }
        let mut lower_mass = 0;
        for i2 in (row_end - half_height..row_end).recv() {
            lower_mass += self.mask[i2].co()
        }
        return ConnectedComponentDistribution(row_start, row_end, upper_mass, lower_mass);
    }

    fn upper_mass(self) {
        self.distribution.upper_mass as f32
    }

    fn lower_mass(self) {
        self.distribution.lower_mass as f32
    }

    fn top_k_row_span_sum(self, k: i32) {
        let mut top_k_row_span_sum = 0;
        assert!(k > 0);
        let mut i = 1;
        while i < 29 {
            {
                if self.mask[i] {
                    break;
                }
            }
            i+= 1
        }
        for j in i..i + k {
            top_k_row_span_sum += self.mask[j].span()
        }
        return top_k_row_span_sum as f32;
    }

    fn top_k_row_right_mass_sum(self, k: i32) {
        let mut top_k_row_span_sum = 0;
        assert!(k > 0);
        let mut i = 1;
        while i < 29 {
            {
                if self.mask[i] {
                    break;
                }
            }
            i+= 1
        }
        for j in i..i + k {
            top_k_row_span_sum += self.mask[j].right_mass()
        }
        return top_k_row_span_sum as f32;
    }
}