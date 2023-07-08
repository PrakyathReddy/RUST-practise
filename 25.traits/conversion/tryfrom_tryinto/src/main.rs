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

impl TryFrom<ApplicationError> for DatabaseError {
    type Error = ApplicationError;

    fn try_from(value: ApplicationError) -> Result<Self, Self::Error> {
        match value {
            ApplicationError::Database(value) => Ok(value),
            _ => Err(value),
        }
    }
}

fn main() {
    // You can use `Into::into` to convert the `DatabaseError` into an `ApplicationError`.
    let app_err: ApplicationError = DatabaseError::ProtocolViolation.into();

    // you can use 'TryInto::try_into' to convert the 'application_error' back into a 'database_error'
    let db_err: DatabaseError = app_err.try_into().unwrap();
    // You can not turn an `ApplicationError::IO` variant into a `DatabaseError`, so this conversion fails.
    let still_app_err: ApplicationError = DatabaseError::try_from(ApplicationError::IO(
        std::io::Error::new(std::io::ErrorKind::Other, "Something bad happened!"),
    ))
    .unwrap_err();
}
