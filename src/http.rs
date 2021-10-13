struct Request {
    caminho: String,
    stringPedido: Option<String>,
    method: Method,                   
}


enum Method {
    GET,    
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}