#[derive(Debug)]
enum DatabaseError {
    DatabaseClosed,
    ProtocolViolation,
}

#[derive(Debug)]
enum ApplicationError {
    Database(DatabaseError),
    IO(std::io::Error),
}

impl From<DatabaseError> for ApplicationError {
    fn from(value: DatabaseError) -> Self {
        Self::Database(value)
    }
}

fn main() {
    // you can use Into::into to convert the DatabaseError into an ApplicationError
    let app_err: ApplicationError = DatabaseError::ProtocolViolation.into();
}
