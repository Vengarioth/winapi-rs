// Copyright © 2015-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Windows HTTP Services API constant definitions and macros
//54
pub type HINTERNET = ::LPVOID;
pub type LPHINTERNET = *mut HINTERNET;
pub type INTERNET_PORT = ::WORD;
pub type LPINTERNET_PORT = *mut INTERNET_PORT;
pub const INTERNET_DEFAULT_PORT: INTERNET_PORT = 0;
pub const INTERNET_DEFAULT_HTTP_PORT: INTERNET_PORT = 80;
pub const INTERNET_DEFAULT_HTTPS_PORT: INTERNET_PORT = 443;
pub const WINHTTP_FLAG_ASYNC: ::DWORD = 0x10000000;
pub const WINHTTP_FLAG_SECURE: ::DWORD = 0x00800000;
pub const WINHTTP_FLAG_ESCAPE_PERCENT: ::DWORD = 0x00000004;
pub const WINHTTP_FLAG_NULL_CODEPAGE: ::DWORD = 0x00000008;
pub const WINHTTP_FLAG_BYPASS_PROXY_CACHE: ::DWORD = 0x00000100;
pub const WINHTTP_FLAG_REFRESH: ::DWORD = WINHTTP_FLAG_BYPASS_PROXY_CACHE;
pub const WINHTTP_FLAG_ESCAPE_DISABLE: ::DWORD = 0x00000040;
pub const WINHTTP_FLAG_ESCAPE_DISABLE_QUERY: ::DWORD = 0x00000080;
STRUCT!{struct WINHTTP_ASYNC_RESULT {
    dwResult: ::DWORD_PTR,
    dwError: ::DWORD,
}}
pub type LPWINHTTP_ASYNC_RESULT = *mut WINHTTP_ASYNC_RESULT;
pub type INTERNET_SCHEME = ::c_int;
pub type LPINTERNET_SCHEME = *mut ::c_int;
pub const INTERNET_SCHEME_HTTP: INTERNET_SCHEME = 1;
pub const INTERNET_SCHEME_HTTPS: INTERNET_SCHEME = 2;
pub const INTERNET_SCHEME_FTP: INTERNET_SCHEME = 3;
pub const INTERNET_SCHEME_SOCKS: INTERNET_SCHEME = 4;
STRUCT!{struct URL_COMPONENTS {
    dwStructSize: ::DWORD,
    lpszScheme: ::LPWSTR,
    dwSchemeLength: ::DWORD,
    nScheme: INTERNET_SCHEME,
    lpszHostName: ::LPWSTR,
    dwHostNameLength: ::DWORD,
    nPort: INTERNET_PORT,
    lpszUserName: ::LPWSTR,
    dwUserNameLength: ::DWORD,
    lpszPassword: ::LPWSTR,
    dwPasswordLength: ::DWORD,
    lpszUrlPath: ::LPWSTR,
    dwUrlPathLength: ::DWORD,
    lpszExtraInfo: ::LPWSTR,
    dwExtraInfoLength: ::DWORD,
}}
pub type LPURL_COMPONENTS = *mut URL_COMPONENTS;
pub type URL_COMPONENTSW = URL_COMPONENTS;
pub type LPURL_COMPONENTSW = LPURL_COMPONENTS;
STRUCT!{struct WINHTTP_PROXY_INFO {
    dwAccessType: ::DWORD,
    lpszProxy: ::LPWSTR,
    lpszProxyBypass: ::LPWSTR,
}}
pub type LPWINHTTP_PROXY_INFO = *mut WINHTTP_PROXY_INFO;
pub type WINHTTP_PROXY_INFOW = WINHTTP_PROXY_INFO;
pub type LPWINHTTP_PROXY_INFOW = LPWINHTTP_PROXY_INFO;
STRUCT!{struct WINHTTP_AUTOPROXY_OPTIONS {
    dwFlags: ::DWORD,
    dwAutoDetectFlags: ::DWORD,
    lpszAutoConfigUrl: ::LPCWSTR,
    lpvReserved: ::LPVOID,
    dwReserved: ::DWORD,
    fAutoLogonIfChallenged: ::BOOL,
}}
pub const WINHTTP_AUTOPROXY_AUTO_DETECT: ::DWORD = 0x00000001;
pub const WINHTTP_AUTOPROXY_CONFIG_URL: ::DWORD = 0x00000002;
pub const WINHTTP_AUTOPROXY_HOST_KEEPCASE: ::DWORD = 0x00000004;
pub const WINHTTP_AUTOPROXY_HOST_LOWERCASE: ::DWORD = 0x00000008;
pub const WINHTTP_AUTOPROXY_RUN_INPROCESS: ::DWORD = 0x00010000;
pub const WINHTTP_AUTOPROXY_RUN_OUTPROCESS_ONLY: ::DWORD = 0x00020000;
pub const WINHTTP_AUTOPROXY_NO_DIRECTACCESS: ::DWORD = 0x00040000;
pub const WINHTTP_AUTOPROXY_NO_CACHE_CLIENT: ::DWORD = 0x00080000;
pub const WINHTTP_AUTOPROXY_NO_CACHE_SVC: ::DWORD = 0x00100000;
pub const WINHTTP_AUTOPROXY_SORT_RESULTS: ::DWORD = 0x00400000;
pub const WINHTTP_AUTO_DETECT_TYPE_DHCP: ::DWORD = 0x00000001;
pub const WINHTTP_AUTO_DETECT_TYPE_DNS_A: ::DWORD = 0x00000002;
STRUCT!{struct WINHTTP_PROXY_RESULT_ENTRY {
    fProxy: ::BOOL,
    fBypass: ::BOOL,
    ProxyScheme: INTERNET_SCHEME,
    pwszProxy: ::PWSTR,
    ProxyPort: INTERNET_PORT,
}}
STRUCT!{struct WINHTTP_PROXY_RESULT {
    cEntries: ::DWORD,
    pEntries: *mut WINHTTP_PROXY_RESULT_ENTRY,
}}
pub const WINHTTP_FIRST_OPTION: ::DWORD = WINHTTP_OPTION_CALLBACK;
pub const WINHTTP_OPTION_CALLBACK: ::DWORD = 1;
pub const WINHTTP_OPTION_RESOLVE_TIMEOUT: ::DWORD = 2;
pub const WINHTTP_OPTION_CONNECT_TIMEOUT: ::DWORD = 3;
pub const WINHTTP_OPTION_CONNECT_RETRIES: ::DWORD = 4;
pub const WINHTTP_OPTION_SEND_TIMEOUT: ::DWORD = 5;
pub const WINHTTP_OPTION_RECEIVE_TIMEOUT: ::DWORD = 6;
pub const WINHTTP_OPTION_RECEIVE_RESPONSE_TIMEOUT: ::DWORD = 7;
pub const WINHTTP_OPTION_HANDLE_TYPE: ::DWORD = 9;
pub const WINHTTP_OPTION_READ_BUFFER_SIZE: ::DWORD = 12;
pub const WINHTTP_OPTION_WRITE_BUFFER_SIZE: ::DWORD = 13;
pub const WINHTTP_OPTION_PARENT_HANDLE: ::DWORD = 21;
pub const WINHTTP_OPTION_EXTENDED_ERROR: ::DWORD = 24;
pub const WINHTTP_OPTION_SECURITY_FLAGS: ::DWORD = 31;
pub const WINHTTP_OPTION_SECURITY_CERTIFICATE_STRUCT: ::DWORD = 32;
pub const WINHTTP_OPTION_URL: ::DWORD = 34;
pub const WINHTTP_OPTION_SECURITY_KEY_BITNESS: ::DWORD = 36;
pub const WINHTTP_OPTION_PROXY: ::DWORD = 38;
pub const WINHTTP_OPTION_PROXY_RESULT_ENTRY: ::DWORD = 39;
pub const WINHTTP_OPTION_USER_AGENT: ::DWORD = 41;
pub const WINHTTP_OPTION_CONTEXT_VALUE: ::DWORD = 45;
pub const WINHTTP_OPTION_CLIENT_CERT_CONTEXT: ::DWORD = 47;
pub const WINHTTP_OPTION_REQUEST_PRIORITY: ::DWORD = 58;
pub const WINHTTP_OPTION_HTTP_VERSION: ::DWORD = 59;
pub const WINHTTP_OPTION_DISABLE_FEATURE: ::DWORD = 63;
pub const WINHTTP_OPTION_CODEPAGE: ::DWORD = 68;
pub const WINHTTP_OPTION_MAX_CONNS_PER_SERVER: ::DWORD = 73;
pub const WINHTTP_OPTION_MAX_CONNS_PER_1_0_SERVER: ::DWORD = 74;
pub const WINHTTP_OPTION_AUTOLOGON_POLICY: ::DWORD = 77;
pub const WINHTTP_OPTION_SERVER_CERT_CONTEXT: ::DWORD = 78;
pub const WINHTTP_OPTION_ENABLE_FEATURE: ::DWORD = 79;
pub const WINHTTP_OPTION_WORKER_THREAD_COUNT: ::DWORD = 80;
pub const WINHTTP_OPTION_PASSPORT_COBRANDING_TEXT: ::DWORD = 81;
pub const WINHTTP_OPTION_PASSPORT_COBRANDING_URL: ::DWORD = 82;
pub const WINHTTP_OPTION_CONFIGURE_PASSPORT_AUTH: ::DWORD = 83;
pub const WINHTTP_OPTION_SECURE_PROTOCOLS: ::DWORD = 84;
pub const WINHTTP_OPTION_ENABLETRACING: ::DWORD = 85;
pub const WINHTTP_OPTION_PASSPORT_SIGN_OUT: ::DWORD = 86;
pub const WINHTTP_OPTION_PASSPORT_RETURN_URL: ::DWORD = 87;
pub const WINHTTP_OPTION_REDIRECT_POLICY: ::DWORD = 88;
pub const WINHTTP_OPTION_MAX_HTTP_AUTOMATIC_REDIRECTS: ::DWORD = 89;
pub const WINHTTP_OPTION_MAX_HTTP_STATUS_CONTINUE: ::DWORD = 90;
pub const WINHTTP_OPTION_MAX_RESPONSE_HEADER_SIZE: ::DWORD = 91;
pub const WINHTTP_OPTION_MAX_RESPONSE_DRAIN_SIZE: ::DWORD = 92;
pub const WINHTTP_OPTION_CONNECTION_INFO: ::DWORD = 93;
pub const WINHTTP_OPTION_CLIENT_CERT_ISSUER_LIST: ::DWORD = 94;
pub const WINHTTP_OPTION_SPN: ::DWORD = 96;
pub const WINHTTP_OPTION_GLOBAL_PROXY_CREDS: ::DWORD = 97;
pub const WINHTTP_OPTION_GLOBAL_SERVER_CREDS: ::DWORD = 98;
pub const WINHTTP_OPTION_UNLOAD_NOTIFY_EVENT: ::DWORD = 99;
pub const WINHTTP_OPTION_REJECT_USERPWD_IN_URL: ::DWORD = 100;
pub const WINHTTP_OPTION_USE_GLOBAL_SERVER_CREDENTIALS: ::DWORD = 101;
pub const WINHTTP_OPTION_RECEIVE_PROXY_CONNECT_RESPONSE: ::DWORD = 103;
pub const WINHTTP_OPTION_IS_PROXY_CONNECT_RESPONSE: ::DWORD = 104;
pub const WINHTTP_OPTION_SERVER_SPN_USED: ::DWORD = 106;
pub const WINHTTP_OPTION_PROXY_SPN_USED: ::DWORD = 107;
pub const WINHTTP_OPTION_SERVER_CBT: ::DWORD = 108;
pub const WINHTTP_OPTION_UNSAFE_HEADER_PARSING: ::DWORD = 110;
pub const WINHTTP_OPTION_ASSURED_NON_BLOCKING_CALLBACKS: ::DWORD = 111;
pub const WINHTTP_OPTION_UPGRADE_TO_WEB_SOCKET: ::DWORD = 114;
pub const WINHTTP_OPTION_WEB_SOCKET_CLOSE_TIMEOUT: ::DWORD = 115;
pub const WINHTTP_OPTION_WEB_SOCKET_KEEPALIVE_INTERVAL: ::DWORD = 116;
pub const WINHTTP_OPTION_DECOMPRESSION: ::DWORD = 118;
pub const WINHTTP_OPTION_WEB_SOCKET_RECEIVE_BUFFER_SIZE: ::DWORD = 122;
pub const WINHTTP_OPTION_WEB_SOCKET_SEND_BUFFER_SIZE: ::DWORD = 123;
pub const WINHTTP_LAST_OPTION: ::DWORD = WINHTTP_OPTION_WEB_SOCKET_SEND_BUFFER_SIZE;
pub const WINHTTP_OPTION_USERNAME: ::DWORD = 0x1000;
pub const WINHTTP_OPTION_PASSWORD: ::DWORD = 0x1001;
pub const WINHTTP_OPTION_PROXY_USERNAME: ::DWORD = 0x1002;
pub const WINHTTP_OPTION_PROXY_PASSWORD: ::DWORD = 0x1003;
//552
FN!{stdcall WINHTTP_STATUS_CALLBACK(
    hInternet: HINTERNET,
    dwContext: ::DWORD_PTR,
    dwInternetStatus: ::DWORD,
    lpvStatusInformation: ::LPVOID,
    dwStatusInformationLength: ::DWORD,
) -> ()}
pub type LPWINHTTP_STATUS_CALLBACK = *mut WINHTTP_STATUS_CALLBACK;
pub const WINHTTP_CALLBACK_STATUS_RESOLVING_NAME: ::DWORD = 0x00000001;
pub const WINHTTP_CALLBACK_STATUS_NAME_RESOLVED: ::DWORD = 0x00000002;
pub const WINHTTP_CALLBACK_STATUS_CONNECTING_TO_SERVER: ::DWORD = 0x00000004;
pub const WINHTTP_CALLBACK_STATUS_CONNECTED_TO_SERVER: ::DWORD = 0x00000008;
pub const WINHTTP_CALLBACK_STATUS_SENDING_REQUEST: ::DWORD = 0x00000010;
pub const WINHTTP_CALLBACK_STATUS_REQUEST_SENT: ::DWORD = 0x00000020;
pub const WINHTTP_CALLBACK_STATUS_RECEIVING_RESPONSE: ::DWORD = 0x00000040;
pub const WINHTTP_CALLBACK_STATUS_RESPONSE_RECEIVED: ::DWORD = 0x00000080;
pub const WINHTTP_CALLBACK_STATUS_CLOSING_CONNECTION: ::DWORD = 0x00000100;
pub const WINHTTP_CALLBACK_STATUS_CONNECTION_CLOSED: ::DWORD = 0x00000200;
pub const WINHTTP_CALLBACK_STATUS_HANDLE_CREATED: ::DWORD = 0x00000400;
pub const WINHTTP_CALLBACK_STATUS_HANDLE_CLOSING: ::DWORD = 0x00000800;
pub const WINHTTP_CALLBACK_STATUS_DETECTING_PROXY: ::DWORD = 0x00001000;
pub const WINHTTP_CALLBACK_STATUS_REDIRECT: ::DWORD = 0x00004000;
pub const WINHTTP_CALLBACK_STATUS_INTERMEDIATE_RESPONSE: ::DWORD = 0x00008000;
pub const WINHTTP_CALLBACK_STATUS_SECURE_FAILURE: ::DWORD = 0x00010000;
pub const WINHTTP_CALLBACK_STATUS_HEADERS_AVAILABLE: ::DWORD = 0x00020000;
pub const WINHTTP_CALLBACK_STATUS_DATA_AVAILABLE: ::DWORD = 0x00040000;
pub const WINHTTP_CALLBACK_STATUS_READ_COMPLETE: ::DWORD = 0x00080000;
pub const WINHTTP_CALLBACK_STATUS_WRITE_COMPLETE: ::DWORD = 0x00100000;
pub const WINHTTP_CALLBACK_STATUS_REQUEST_ERROR: ::DWORD = 0x00200000;
pub const WINHTTP_CALLBACK_STATUS_SENDREQUEST_COMPLETE: ::DWORD = 0x00400000;
pub const WINHTTP_CALLBACK_STATUS_GETPROXYFORURL_COMPLETE: ::DWORD = 0x01000000;
pub const WINHTTP_CALLBACK_STATUS_CLOSE_COMPLETE: ::DWORD = 0x02000000;
pub const WINHTTP_CALLBACK_STATUS_SHUTDOWN_COMPLETE: ::DWORD = 0x04000000;
pub const WINHTTP_CALLBACK_FLAG_RESOLVE_NAME: ::DWORD = WINHTTP_CALLBACK_STATUS_RESOLVING_NAME
    | WINHTTP_CALLBACK_STATUS_NAME_RESOLVED;
pub const WINHTTP_CALLBACK_FLAG_CONNECT_TO_SERVER: ::DWORD =
    WINHTTP_CALLBACK_STATUS_CONNECTING_TO_SERVER | WINHTTP_CALLBACK_STATUS_CONNECTED_TO_SERVER;
pub const WINHTTP_CALLBACK_FLAG_SEND_REQUEST: ::DWORD =
    WINHTTP_CALLBACK_STATUS_SENDING_REQUEST | WINHTTP_CALLBACK_STATUS_REQUEST_SENT;
pub const WINHTTP_CALLBACK_FLAG_RECEIVE_RESPONSE: ::DWORD =
    WINHTTP_CALLBACK_STATUS_RECEIVING_RESPONSE | WINHTTP_CALLBACK_STATUS_RESPONSE_RECEIVED;
pub const WINHTTP_CALLBACK_FLAG_CLOSE_CONNECTION: ::DWORD =
    WINHTTP_CALLBACK_STATUS_CLOSING_CONNECTION | WINHTTP_CALLBACK_STATUS_CONNECTION_CLOSED;
pub const WINHTTP_CALLBACK_FLAG_HANDLES: ::DWORD =
    WINHTTP_CALLBACK_STATUS_HANDLE_CREATED | WINHTTP_CALLBACK_STATUS_HANDLE_CLOSING;
pub const WINHTTP_CALLBACK_FLAG_DETECTING_PROXY: ::DWORD = WINHTTP_CALLBACK_STATUS_DETECTING_PROXY;
pub const WINHTTP_CALLBACK_FLAG_REDIRECT: ::DWORD = WINHTTP_CALLBACK_STATUS_REDIRECT;
pub const WINHTTP_CALLBACK_FLAG_INTERMEDIATE_RESPONSE: ::DWORD =
    WINHTTP_CALLBACK_STATUS_INTERMEDIATE_RESPONSE;
pub const WINHTTP_CALLBACK_FLAG_SECURE_FAILURE: ::DWORD = WINHTTP_CALLBACK_STATUS_SECURE_FAILURE;
pub const WINHTTP_CALLBACK_FLAG_SENDREQUEST_COMPLETE: ::DWORD =
    WINHTTP_CALLBACK_STATUS_SENDREQUEST_COMPLETE;
pub const WINHTTP_CALLBACK_FLAG_HEADERS_AVAILABLE: ::DWORD =
    WINHTTP_CALLBACK_STATUS_HEADERS_AVAILABLE;
pub const WINHTTP_CALLBACK_FLAG_DATA_AVAILABLE: ::DWORD = WINHTTP_CALLBACK_STATUS_DATA_AVAILABLE;
pub const WINHTTP_CALLBACK_FLAG_READ_COMPLETE: ::DWORD = WINHTTP_CALLBACK_STATUS_READ_COMPLETE;
pub const WINHTTP_CALLBACK_FLAG_WRITE_COMPLETE: ::DWORD = WINHTTP_CALLBACK_STATUS_WRITE_COMPLETE;
pub const WINHTTP_CALLBACK_FLAG_REQUEST_ERROR: ::DWORD = WINHTTP_CALLBACK_STATUS_REQUEST_ERROR;
pub const WINHTTP_CALLBACK_FLAG_GETPROXYFORURL_COMPLETE: ::DWORD =
    WINHTTP_CALLBACK_STATUS_GETPROXYFORURL_COMPLETE;
pub const WINHTTP_CALLBACK_FLAG_ALL_COMPLETIONS: ::DWORD =
    WINHTTP_CALLBACK_STATUS_SENDREQUEST_COMPLETE | WINHTTP_CALLBACK_STATUS_HEADERS_AVAILABLE
    | WINHTTP_CALLBACK_STATUS_DATA_AVAILABLE | WINHTTP_CALLBACK_STATUS_READ_COMPLETE
    | WINHTTP_CALLBACK_STATUS_WRITE_COMPLETE | WINHTTP_CALLBACK_STATUS_REQUEST_ERROR
    | WINHTTP_CALLBACK_STATUS_GETPROXYFORURL_COMPLETE;
pub const WINHTTP_CALLBACK_FLAG_ALL_NOTIFICATIONS: ::DWORD = 0xffffffff;
pub const WINHTTP_QUERY_MIME_VERSION: ::DWORD = 0;
pub const WINHTTP_QUERY_CONTENT_TYPE: ::DWORD = 1;
pub const WINHTTP_QUERY_CONTENT_TRANSFER_ENCODING: ::DWORD = 2;
pub const WINHTTP_QUERY_CONTENT_ID: ::DWORD = 3;
pub const WINHTTP_QUERY_CONTENT_DESCRIPTION: ::DWORD = 4;
pub const WINHTTP_QUERY_CONTENT_LENGTH: ::DWORD = 5;
pub const WINHTTP_QUERY_CONTENT_LANGUAGE: ::DWORD = 6;
pub const WINHTTP_QUERY_ALLOW: ::DWORD = 7;
pub const WINHTTP_QUERY_PUBLIC: ::DWORD = 8;
pub const WINHTTP_QUERY_DATE: ::DWORD = 9;
pub const WINHTTP_QUERY_EXPIRES: ::DWORD = 10;
pub const WINHTTP_QUERY_LAST_MODIFIED: ::DWORD = 11;
pub const WINHTTP_QUERY_MESSAGE_ID: ::DWORD = 12;
pub const WINHTTP_QUERY_URI: ::DWORD = 13;
pub const WINHTTP_QUERY_DERIVED_FROM: ::DWORD = 14;
pub const WINHTTP_QUERY_COST: ::DWORD = 15;
pub const WINHTTP_QUERY_LINK: ::DWORD = 16;
pub const WINHTTP_QUERY_PRAGMA: ::DWORD = 17;
pub const WINHTTP_QUERY_VERSION: ::DWORD = 18;
pub const WINHTTP_QUERY_STATUS_CODE: ::DWORD = 19;
pub const WINHTTP_QUERY_STATUS_TEXT: ::DWORD = 20;
pub const WINHTTP_QUERY_RAW_HEADERS: ::DWORD = 21;
pub const WINHTTP_QUERY_RAW_HEADERS_CRLF: ::DWORD = 22;
pub const WINHTTP_QUERY_CONNECTION: ::DWORD = 23;
pub const WINHTTP_QUERY_ACCEPT: ::DWORD = 24;
pub const WINHTTP_QUERY_ACCEPT_CHARSET: ::DWORD = 25;
pub const WINHTTP_QUERY_ACCEPT_ENCODING: ::DWORD = 26;
pub const WINHTTP_QUERY_ACCEPT_LANGUAGE: ::DWORD = 27;
pub const WINHTTP_QUERY_AUTHORIZATION: ::DWORD = 28;
pub const WINHTTP_QUERY_CONTENT_ENCODING: ::DWORD = 29;
pub const WINHTTP_QUERY_FORWARDED: ::DWORD = 30;
pub const WINHTTP_QUERY_FROM: ::DWORD = 31;
pub const WINHTTP_QUERY_IF_MODIFIED_SINCE: ::DWORD = 32;
pub const WINHTTP_QUERY_LOCATION: ::DWORD = 33;
pub const WINHTTP_QUERY_ORIG_URI: ::DWORD = 34;
pub const WINHTTP_QUERY_REFERER: ::DWORD = 35;
pub const WINHTTP_QUERY_RETRY_AFTER: ::DWORD = 36;
pub const WINHTTP_QUERY_SERVER: ::DWORD = 37;
pub const WINHTTP_QUERY_TITLE: ::DWORD = 38;
pub const WINHTTP_QUERY_USER_AGENT: ::DWORD = 39;
pub const WINHTTP_QUERY_WWW_AUTHENTICATE: ::DWORD = 40;
pub const WINHTTP_QUERY_PROXY_AUTHENTICATE: ::DWORD = 41;
pub const WINHTTP_QUERY_ACCEPT_RANGES: ::DWORD = 42;
pub const WINHTTP_QUERY_SET_COOKIE: ::DWORD = 43;
pub const WINHTTP_QUERY_COOKIE: ::DWORD = 44;
pub const WINHTTP_QUERY_REQUEST_METHOD: ::DWORD = 45;
pub const WINHTTP_QUERY_REFRESH: ::DWORD = 46;
pub const WINHTTP_QUERY_CONTENT_DISPOSITION: ::DWORD = 47;
pub const WINHTTP_QUERY_AGE: ::DWORD = 48;
pub const WINHTTP_QUERY_CACHE_CONTROL: ::DWORD = 49;
pub const WINHTTP_QUERY_CONTENT_BASE: ::DWORD = 50;
pub const WINHTTP_QUERY_CONTENT_LOCATION: ::DWORD = 51;
pub const WINHTTP_QUERY_CONTENT_MD5: ::DWORD = 52;
pub const WINHTTP_QUERY_CONTENT_RANGE: ::DWORD = 53;
pub const WINHTTP_QUERY_ETAG: ::DWORD = 54;
pub const WINHTTP_QUERY_HOST: ::DWORD = 55;
pub const WINHTTP_QUERY_IF_MATCH: ::DWORD = 56;
pub const WINHTTP_QUERY_IF_NONE_MATCH: ::DWORD = 57;
pub const WINHTTP_QUERY_IF_RANGE: ::DWORD = 58;
pub const WINHTTP_QUERY_IF_UNMODIFIED_SINCE: ::DWORD = 59;
pub const WINHTTP_QUERY_MAX_FORWARDS: ::DWORD = 60;
pub const WINHTTP_QUERY_PROXY_AUTHORIZATION: ::DWORD = 61;
pub const WINHTTP_QUERY_RANGE: ::DWORD = 62;
pub const WINHTTP_QUERY_TRANSFER_ENCODING: ::DWORD = 63;
pub const WINHTTP_QUERY_UPGRADE: ::DWORD = 64;
pub const WINHTTP_QUERY_VARY: ::DWORD = 65;
pub const WINHTTP_QUERY_VIA: ::DWORD = 66;
pub const WINHTTP_QUERY_WARNING: ::DWORD = 67;
pub const WINHTTP_QUERY_EXPECT: ::DWORD = 68;
pub const WINHTTP_QUERY_PROXY_CONNECTION: ::DWORD = 69;
pub const WINHTTP_QUERY_UNLESS_MODIFIED_SINCE: ::DWORD = 70;
pub const WINHTTP_QUERY_PROXY_SUPPORT: ::DWORD = 75;
pub const WINHTTP_QUERY_AUTHENTICATION_INFO: ::DWORD = 76;
pub const WINHTTP_QUERY_PASSPORT_URLS: ::DWORD = 77;
pub const WINHTTP_QUERY_PASSPORT_CONFIG: ::DWORD = 78;
pub const WINHTTP_QUERY_MAX: ::DWORD = 78;
pub const WINHTTP_QUERY_CUSTOM: ::DWORD = 65535;
pub const WINHTTP_QUERY_FLAG_REQUEST_HEADERS: ::DWORD = 0x80000000;
pub const WINHTTP_QUERY_FLAG_SYSTEMTIME: ::DWORD = 0x40000000;
pub const WINHTTP_QUERY_FLAG_NUMBER: ::DWORD = 0x20000000;
pub const HTTP_STATUS_CONTINUE: ::DWORD = 100;
pub const HTTP_STATUS_SWITCH_PROTOCOLS: ::DWORD = 101;
pub const HTTP_STATUS_OK: ::DWORD = 200;
pub const HTTP_STATUS_CREATED: ::DWORD = 201;
pub const HTTP_STATUS_ACCEPTED: ::DWORD = 202;
pub const HTTP_STATUS_PARTIAL: ::DWORD = 203;
pub const HTTP_STATUS_NO_CONTENT: ::DWORD = 204;
pub const HTTP_STATUS_RESET_CONTENT: ::DWORD = 205;
pub const HTTP_STATUS_PARTIAL_CONTENT: ::DWORD = 206;
pub const HTTP_STATUS_WEBDAV_MULTI_STATUS: ::DWORD = 207;
pub const HTTP_STATUS_AMBIGUOUS: ::DWORD = 300;
pub const HTTP_STATUS_MOVED: ::DWORD = 301;
pub const HTTP_STATUS_REDIRECT: ::DWORD = 302;
pub const HTTP_STATUS_REDIRECT_METHOD: ::DWORD = 303;
pub const HTTP_STATUS_NOT_MODIFIED: ::DWORD = 304;
pub const HTTP_STATUS_USE_PROXY: ::DWORD = 305;
pub const HTTP_STATUS_REDIRECT_KEEP_VERB: ::DWORD = 307;
pub const HTTP_STATUS_BAD_REQUEST: ::DWORD = 400;
pub const HTTP_STATUS_DENIED: ::DWORD = 401;
pub const HTTP_STATUS_PAYMENT_REQ: ::DWORD = 402;
pub const HTTP_STATUS_FORBIDDEN: ::DWORD = 403;
pub const HTTP_STATUS_NOT_FOUND: ::DWORD = 404;
pub const HTTP_STATUS_BAD_METHOD: ::DWORD = 405;
pub const HTTP_STATUS_NONE_ACCEPTABLE: ::DWORD = 406;
pub const HTTP_STATUS_PROXY_AUTH_REQ: ::DWORD = 407;
pub const HTTP_STATUS_REQUEST_TIMEOUT: ::DWORD = 408;
pub const HTTP_STATUS_CONFLICT: ::DWORD = 409;
pub const HTTP_STATUS_GONE: ::DWORD = 410;
pub const HTTP_STATUS_LENGTH_REQUIRED: ::DWORD = 411;
pub const HTTP_STATUS_PRECOND_FAILED: ::DWORD = 412;
pub const HTTP_STATUS_REQUEST_TOO_LARGE: ::DWORD = 413;
pub const HTTP_STATUS_URI_TOO_LONG: ::DWORD = 414;
pub const HTTP_STATUS_UNSUPPORTED_MEDIA: ::DWORD = 415;
pub const HTTP_STATUS_RETRY_WITH: ::DWORD = 449;
pub const HTTP_STATUS_SERVER_ERROR: ::DWORD = 500;
pub const HTTP_STATUS_NOT_SUPPORTED: ::DWORD = 501;
pub const HTTP_STATUS_BAD_GATEWAY: ::DWORD = 502;
pub const HTTP_STATUS_SERVICE_UNAVAIL: ::DWORD = 503;
pub const HTTP_STATUS_GATEWAY_TIMEOUT: ::DWORD = 504;
pub const HTTP_STATUS_VERSION_NOT_SUP: ::DWORD = 505;
pub const HTTP_STATUS_FIRST: ::DWORD = HTTP_STATUS_CONTINUE;
pub const HTTP_STATUS_LAST: ::DWORD = HTTP_STATUS_VERSION_NOT_SUP;
pub const WINHTTP_ACCESS_TYPE_DEFAULT_PROXY: ::DWORD = 0;
pub const WINHTTP_ACCESS_TYPE_NO_PROXY: ::DWORD = 1;
pub const WINHTTP_ACCESS_TYPE_NAMED_PROXY: ::DWORD = 3;
pub const WINHTTP_ACCESS_TYPE_AUTOMATIC_PROXY: ::DWORD = 4;
pub const WINHTTP_ERROR_BASE: ::DWORD = 12000;
pub const ERROR_WINHTTP_OUT_OF_HANDLES: ::DWORD = WINHTTP_ERROR_BASE + 1;
pub const ERROR_WINHTTP_TIMEOUT: ::DWORD = WINHTTP_ERROR_BASE + 2;
pub const ERROR_WINHTTP_INTERNAL_ERROR: ::DWORD = WINHTTP_ERROR_BASE + 4;
pub const ERROR_WINHTTP_INVALID_URL: ::DWORD = WINHTTP_ERROR_BASE + 5;
pub const ERROR_WINHTTP_UNRECOGNIZED_SCHEME: ::DWORD = WINHTTP_ERROR_BASE + 6;
pub const ERROR_WINHTTP_NAME_NOT_RESOLVED: ::DWORD = WINHTTP_ERROR_BASE + 7;
pub const ERROR_WINHTTP_INVALID_OPTION: ::DWORD = WINHTTP_ERROR_BASE + 9;
pub const ERROR_WINHTTP_OPTION_NOT_SETTABLE: ::DWORD = WINHTTP_ERROR_BASE + 11;
pub const ERROR_WINHTTP_SHUTDOWN: ::DWORD = WINHTTP_ERROR_BASE + 12;
pub const ERROR_WINHTTP_LOGIN_FAILURE: ::DWORD = WINHTTP_ERROR_BASE + 15;
pub const ERROR_WINHTTP_OPERATION_CANCELLED: ::DWORD = WINHTTP_ERROR_BASE + 17;
pub const ERROR_WINHTTP_INCORRECT_HANDLE_TYPE: ::DWORD = WINHTTP_ERROR_BASE + 18;
pub const ERROR_WINHTTP_INCORRECT_HANDLE_STATE: ::DWORD = WINHTTP_ERROR_BASE + 19;
pub const ERROR_WINHTTP_CANNOT_CONNECT: ::DWORD = WINHTTP_ERROR_BASE + 29;
pub const ERROR_WINHTTP_CONNECTION_ERROR: ::DWORD = WINHTTP_ERROR_BASE + 30;
pub const ERROR_WINHTTP_RESEND_REQUEST: ::DWORD = WINHTTP_ERROR_BASE + 32;
pub const ERROR_WINHTTP_CLIENT_AUTH_CERT_NEEDED: ::DWORD = WINHTTP_ERROR_BASE + 44;
pub const ERROR_WINHTTP_CANNOT_CALL_BEFORE_OPEN: ::DWORD = WINHTTP_ERROR_BASE + 100;
pub const ERROR_WINHTTP_CANNOT_CALL_BEFORE_SEND: ::DWORD = WINHTTP_ERROR_BASE + 101;
pub const ERROR_WINHTTP_CANNOT_CALL_AFTER_SEND: ::DWORD = WINHTTP_ERROR_BASE + 102;
pub const ERROR_WINHTTP_CANNOT_CALL_AFTER_OPEN: ::DWORD = WINHTTP_ERROR_BASE + 103;
pub const ERROR_WINHTTP_HEADER_NOT_FOUND: ::DWORD = WINHTTP_ERROR_BASE + 150;
pub const ERROR_WINHTTP_INVALID_SERVER_RESPONSE: ::DWORD = WINHTTP_ERROR_BASE + 152;
pub const ERROR_WINHTTP_INVALID_HEADER: ::DWORD = WINHTTP_ERROR_BASE + 153;
pub const ERROR_WINHTTP_INVALID_QUERY_REQUEST: ::DWORD = WINHTTP_ERROR_BASE + 154;
pub const ERROR_WINHTTP_HEADER_ALREADY_EXISTS: ::DWORD = WINHTTP_ERROR_BASE + 155;
pub const ERROR_WINHTTP_REDIRECT_FAILED: ::DWORD = WINHTTP_ERROR_BASE + 156;
pub const ERROR_WINHTTP_AUTO_PROXY_SERVICE_ERROR: ::DWORD = WINHTTP_ERROR_BASE + 178;
pub const ERROR_WINHTTP_BAD_AUTO_PROXY_SCRIPT: ::DWORD = WINHTTP_ERROR_BASE + 166;
pub const ERROR_WINHTTP_UNABLE_TO_DOWNLOAD_SCRIPT: ::DWORD = WINHTTP_ERROR_BASE + 167;
pub const ERROR_WINHTTP_UNHANDLED_SCRIPT_TYPE: ::DWORD = WINHTTP_ERROR_BASE + 176;
pub const ERROR_WINHTTP_SCRIPT_EXECUTION_ERROR: ::DWORD = WINHTTP_ERROR_BASE + 177;
pub const ERROR_WINHTTP_NOT_INITIALIZED: ::DWORD = WINHTTP_ERROR_BASE + 172;
pub const ERROR_WINHTTP_SECURE_FAILURE: ::DWORD = WINHTTP_ERROR_BASE + 175;
pub const ERROR_WINHTTP_SECURE_CERT_DATE_INVALID: ::DWORD = WINHTTP_ERROR_BASE + 37;
pub const ERROR_WINHTTP_SECURE_CERT_CN_INVALID: ::DWORD = WINHTTP_ERROR_BASE + 38;
pub const ERROR_WINHTTP_SECURE_INVALID_CA: ::DWORD = WINHTTP_ERROR_BASE + 45;
pub const ERROR_WINHTTP_SECURE_CERT_REV_FAILED: ::DWORD = WINHTTP_ERROR_BASE + 57;
pub const ERROR_WINHTTP_SECURE_CHANNEL_ERROR: ::DWORD = WINHTTP_ERROR_BASE + 157;
pub const ERROR_WINHTTP_SECURE_INVALID_CERT: ::DWORD = WINHTTP_ERROR_BASE + 169;
pub const ERROR_WINHTTP_SECURE_CERT_REVOKED: ::DWORD = WINHTTP_ERROR_BASE + 170;
pub const ERROR_WINHTTP_SECURE_CERT_WRONG_USAGE: ::DWORD = WINHTTP_ERROR_BASE + 179;
pub const ERROR_WINHTTP_AUTODETECTION_FAILED: ::DWORD = WINHTTP_ERROR_BASE + 180;
pub const ERROR_WINHTTP_HEADER_COUNT_EXCEEDED: ::DWORD = WINHTTP_ERROR_BASE + 181;
pub const ERROR_WINHTTP_HEADER_SIZE_OVERFLOW: ::DWORD = WINHTTP_ERROR_BASE + 182;
pub const ERROR_WINHTTP_CHUNKED_ENCODING_HEADER_SIZE_OVERFLOW: ::DWORD = WINHTTP_ERROR_BASE + 183;
pub const ERROR_WINHTTP_RESPONSE_DRAIN_OVERFLOW: ::DWORD = WINHTTP_ERROR_BASE + 184;
pub const ERROR_WINHTTP_CLIENT_CERT_NO_PRIVATE_KEY: ::DWORD = WINHTTP_ERROR_BASE + 185;
pub const ERROR_WINHTTP_CLIENT_CERT_NO_ACCESS_PRIVATE_KEY: ::DWORD = WINHTTP_ERROR_BASE + 186;
pub const WINHTTP_ERROR_LAST: ::DWORD = WINHTTP_ERROR_BASE + 186;
pub const WINHTTP_RESET_STATE: ::DWORD = 0x00000001;
pub const WINHTTP_RESET_SWPAD_CURRENT_NETWORK: ::DWORD = 0x00000002;
pub const WINHTTP_RESET_SWPAD_ALL: ::DWORD = 0x00000004;
pub const WINHTTP_RESET_SCRIPT_CACHE: ::DWORD = 0x00000008;
pub const WINHTTP_RESET_ALL: ::DWORD = 0x0000FFFF;
pub const WINHTTP_RESET_NOTIFY_NETWORK_CHANGED: ::DWORD = 0x00010000;
pub const WINHTTP_RESET_OUT_OF_PROC: ::DWORD = 0x00020000;
STRUCT!{struct WINHTTP_CURRENT_USER_IE_PROXY_CONFIG {
    fAutoDetect: ::BOOL,
    lpszAutoConfigUrl: ::LPWSTR,
    lpszProxy: ::LPWSTR,
    lpszProxyBypass: ::LPWSTR,
}}
//1370
ENUM!{enum WINHTTP_WEB_SOCKET_OPERATION {
    WINHTTP_WEB_SOCKET_SEND_OPERATION = 0,
    WINHTTP_WEB_SOCKET_RECEIVE_OPERATION = 1,
    WINHTTP_WEB_SOCKET_CLOSE_OPERATION = 2,
    WINHTTP_WEB_SOCKET_SHUTDOWN_OPERATION = 3,
}}
ENUM!{enum WINHTTP_WEB_SOCKET_BUFFER_TYPE {
    WINHTTP_WEB_SOCKET_BINARY_MESSAGE_BUFFER_TYPE = 0,
    WINHTTP_WEB_SOCKET_BINARY_FRAGMENT_BUFFER_TYPE = 1,
    WINHTTP_WEB_SOCKET_UTF8_MESSAGE_BUFFER_TYPE = 2,
    WINHTTP_WEB_SOCKET_UTF8_FRAGMENT_BUFFER_TYPE = 3,
    WINHTTP_WEB_SOCKET_CLOSE_BUFFER_TYPE = 4,
}}
ENUM!{enum WINHTTP_WEB_SOCKET_CLOSE_STATUS {
    WINHTTP_WEB_SOCKET_SUCCESS_CLOSE_STATUS = 1000,
    WINHTTP_WEB_SOCKET_ENDPOINT_TERMINATED_CLOSE_STATUS = 1001,
    WINHTTP_WEB_SOCKET_PROTOCOL_ERROR_CLOSE_STATUS = 1002,
    WINHTTP_WEB_SOCKET_INVALID_DATA_TYPE_CLOSE_STATUS = 1003,
    WINHTTP_WEB_SOCKET_EMPTY_CLOSE_STATUS = 1005,
    WINHTTP_WEB_SOCKET_ABORTED_CLOSE_STATUS = 1006,
    WINHTTP_WEB_SOCKET_INVALID_PAYLOAD_CLOSE_STATUS = 1007,
    WINHTTP_WEB_SOCKET_POLICY_VIOLATION_CLOSE_STATUS = 1008,
    WINHTTP_WEB_SOCKET_MESSAGE_TOO_BIG_CLOSE_STATUS = 1009,
    WINHTTP_WEB_SOCKET_UNSUPPORTED_EXTENSIONS_CLOSE_STATUS = 1010,
    WINHTTP_WEB_SOCKET_SERVER_ERROR_CLOSE_STATUS = 1011,
    WINHTTP_WEB_SOCKET_SECURE_HANDSHAKE_ERROR_CLOSE_STATUS = 1015,
}}
