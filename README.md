# quick-pull
Quickly retrieves a stored image.

<br/>

# Architecture

  |     Datastore     |        | Serverless Image API |         |      Img url can be used for      |
  |                   |   ->   |                      |.  ->.   |    prod use. Loads super fast.    |
  |   Aerospike  DB   |        |    Rust on Fermyon   |         | Example in HTML, yew, react, etc. |
