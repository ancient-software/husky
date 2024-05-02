use super::connection::MajorItemConnection;

pub(super) fn show_connection(connection: MajorItemConnection) -> &'static str {
    match connection {
        MajorItemConnection::Connected => "::",
        MajorItemConnection::Disconnected(_) => ":/:",
    }
}
