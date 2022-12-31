# Specification

## Introduction

The *IP Service Protocol* (IPSP) is a TCP service that accepts connections on TCP port 5350 (the hexadecimal representation of the ASCII string `SP`).

The purpose of IPSP is to provide metadata about another TCP service, which may include the privacy policy, terms of service, and contact information. This provides prospective users of the TCP service an opportunity to review their rights and responsibilities before establishing a connection.

IPSP is designed to be simple to understand, simple to implement, and inexstensible.

IPSP is a client-server protocol. The *IPSP client* or *client*  is the host that initiates the TCP connection. The *IPSP server* or *server* is the host that the client connects to.

## Basic Definitions

* **IPSP-conforming implementation**, **IPSP implementation**, or simply **implementation**
  * A host that conforms to an IPSP client or IPSP server role.
* **target service**
  * The TCP service provided by an IPSP server that is the subject of the [Privacy Policy](#privacy-policy), [Terms of Service](#terms-of-service), and [Contact Information](#contact-information) queries.
* **MUST (do something)**
  * An IPSP-conforming implementation will do this.
  > *Informative.* A host that does not do as prescribed by the IPSP client or server roles cannot be considered an IPSP-conforming implementation and is out of the scope of this specification.
* **MUST NOT (do something)**
  * An implementation will not do this.
* **SHOULD (do something)**
  * An IPSP-conforming implementation may or may not do this, but it is preferred that the implementation does this.
  > *Informative.* Actions that would otherwise be qualified with MUST are sometimes instead qualified with SHOULD due to the difficulty of rigorously defining the action.
* **SHOULD NOT (do something)**
  * An implementation may or may not do this, but it is preferred to not do this.

## Communication

After a successful TCP handshake and until a TCP connection is terminated, the client MUST only send TCP traffic to the server in the form of *queries*. The server MUST only send TCP traffic to the client in response to queries as *query responses* or *responses*.

> *Informative. Negation.* The server MUST NOT respond to TCP traffic sent by the client that does not conform to queries.

There are no restrictions on timeouts for sending or receiving TCP segments.

### Client

A query is an octet sent in one TCP segment from a client to a server while a TCP connection is established that matches one of the following:

* [**IPSP Version**](#ipsp-version)
  * Encoded as 118 (decimal), `0x76` (hexadecimal), or `v` (ASCII).
* [**Privacy Policy**](#privacy-policy)
  * Encoded as 112 (decimal), `0x70` (hexadecimal), or `p` (ASCII).
* [**Terms of Service**](#terms-of-service)
  * Encoded as 116 (decimal), `0x74` (hexadecimal), or `t` (ASCII).
* [**Contact Information**](#contact-information)
  * Encoded as 99 (decimal), `0x63` (hexadecimal), or `c` (ASCII).

In a single TCP connection, a client may send multiple queries in any order but MUST only send each query zero or one times. A server MUST NOT respond to duplicate queries.

> *Informative.* A server MUST ignore the second `p` query in the sequence `p`, `t`, and `p`. If another `t` query is then sent, the server MUST ignore that, too.

Multiple queries may be sent in individual TCP segments or combined in a single segment. Either way, sequences of queries are not delimited or terminated, and the server MUST send responses in the order that the queries were sent.

> *Informative.*  For example, the queries `p`, `t`, and `c` could be sent in a single TCP segment as `ptc` or sent in multiple segments as `pt` and `c`, `p` and `tc`, or individually as `p`, `t`, and `c`. In any case, the server MUST respond with the Privacy Policy first, the Terms of Service second, and the Contact Information third.

### Server

One query warrants zero or one responses from the server. A server MUST encode query responses in UTF-8 and MUST terminate each response with the NUL octet. A response may be sent in one or more TCP segments. A sequence of two or more responses may be sent in individual TCP segments or combined in a single segment.

A server MUST read the first four octets sent by a client but may elect to ignore subsequent octets, including valid queries. A server MUST attempt to terminate the TCP connection after responses have been sent for all queries that were read by the server.

#### Query Responses

##### IPSP Version

The server MUST respond with a version string that identifies the revision of the IP Service Protocol it implements. This version string is the `<version core>` rule described by [the Semantic Versioning 2.0.0 specification](https://semver.org/spec/v2.0.0.html). The response does not contain any whitespace.

The IPSP version defined by this specification is `0.1.0`.

##### Privacy Policy

The server SHOULD respond with the privacy policy of the target service. The response SHOULD be interpeted as the `text/plain` MIME type and SHOULD be readable by a human.

##### Terms of Service

The server SHOULD respond with the terms of service of the target service. The response SHOULD be interpeted as the `text/plain` MIME type and SHOULD be readable by a human.

##### Contact Information

The server SHOULD respond with an email address to which a prospective user of the target service might direct questions regarding the service.
