// use psl::Datasource;

/*
// check if we can connect to the database
// if true: return true
// if false: throw error
export async function ensureCanConnectToDatabase(datasource: DataSource | undefined): Promise<Boolean | Error> {
  if (!datasource) {
    throw new Error(`A datasource block is missing in the Prisma schema file.`)
  }

  const schemaDir = path.dirname(datasource.sourceFilePath)
  const url = getConnectionUrl(datasource)

  // url exists because `ignoreEnvVarErrors: false` would have thrown an error if not
  const canConnect = await canConnectToDatabase(url, schemaDir)

  if (canConnect === true) {
    return true
  } else {
    const { code, message } = canConnect
    throw new Error(`${code}: ${message}`)
  }

}


function getConnectionUrl(datasource: DataSource): string {
  const url = getEffectiveUrl(datasource)
  if (!url.value) {
    if (url.fromEnvVar) {
      throw new Error(`Environment variable '${url.fromEnvVar}' with database connection URL was not found.`)
    } else {
      throw new Error(`Datasource is missing a database connection URL.`)
    }
  }
  return url.value
}

export function getEffectiveUrl(ds: DataSource): EnvValue {
  if (ds.directUrl !== undefined) return ds.directUrl

  return ds.url
}
 */

use colored::Colorize;
use psl::Configuration;
use psl_schema::Schema;

use crate::url::parse_connection_string;

/// Prints information about the current datasource.
///
/// Example:
///   Datasource "db": SQLite database "dev.db" at "file:./dev.db"
///   Datasource "my_db": PostgreSQL database "tests-migrate", schema "public" at "localhost:5432"
///   Datasource "my_db": MySQL database "tests-migrate" at "localhost:5432"
///   Datasource "my_db": MongoDB database "tests-migrate" at "localhost:27017"
///   Datasource "my_db": SQL Server database
pub fn print_datasource(config: &Configuration) {
    let datasource = config.first_datasource();
    let connection = parse_connection_string(datasource.url.as_literal().unwrap());

    let name = &datasource.name;
    let provider = pretty_provider(&datasource.provider);

    let db_name = if let Some(database) = connection.database {
        format!(" \"{database}\"")
    } else {
        "".to_owned()
    };

    println!(
        "{}",
        format!("Datasource \"{name}\": {provider} database{db_name}").dimmed()
    );
}

/*

  // If schemas are defined in the datasource block, print them
  if (datasourceInfo.schemas?.length) {
    message += `, schemas "${datasourceInfo.schemas.join(', ')}"`
  }
  // Otherwise, print the schema if it's defined in the connection string
  else if (datasourceInfo.schema) {
    message += `, schema "${datasourceInfo.schema}"`
  }

  if (adapter) {
    message += ` using driver adapter "${adapter.adapterName}"`
  } else if (datasourceInfo.dbLocation) {
    message += ` at "${datasourceInfo.dbLocation}"`
  }

  process.stdout.write(dim(message) + '\n')
}

*/

/// Turns the internal provider name into an externally viewable "pretty" provider.
///
/// # Example
///
/// ```rust
/// assert_eq!(pretty_provider("mysql"), "MySQL");
/// ```
pub fn pretty_provider(provider: &str) -> &'static str {
    match provider {
        "mysql" => "MySQL",
        "postgres" | "postgresql" => "PostgreSQL",
        "sqlite" => "SQLite",
        _ => panic!("unknown provider {provider}"),
    }
}

pub trait Pluralize {
    fn pluralize(&self, singular: &'static str, plural: &'static str) -> &'static str;
}

impl<T> Pluralize for Vec<T> {
    fn pluralize(&self, singular: &'static str, plural: &'static str) -> &'static str {
        if self.len() == 0 { singular } else { plural }
    }
}
