# RDSync DataBase
Created by LinearDev

----

## Transactional Pipelining Protocol (TPP)
### Introduction
This protocol is designed for asynchronous query queuing and transaction pipelining in databases and other performance-intensive storage environments.

The goals of the protocol are:
 * Scalability. Allows scaling up the number of concurrent clients and transactions by utilizing query queues and thread pooling for processing.
 * Low latency. Clients can send data asynchronously without waiting for requests and responses to be processed by the server.
 * High availability. Separation of client and server parts into separate processes increases system fault tolerance.
 * Horizontal scaling. The architecture of queues and worker-flows allows you to easily distribute the load between several servers.
 * Versatility. The protocol does not depend on the type of data storage and allows processing arbitrary transactions for different databases, services and storages.

Thus, this approach solves the problem of scaling the processing of large transaction loads for modern highly loaded systems.

### Sending a transaction by the client

The client forms the transaction body as a string according to the internal storage format
The client also specifies mandatory transaction headers, for example:
* Query type (add, update, select, etc.)
* Database name
* Table or collection name
* Record identifier
  The headers and body are combined into a single transaction.

The transaction is divided into 2 parts - header and body, using special delimiter bytes:
 * 0x01 - the beginning of the header
 * 0x02 - the beginning of the body
 * 0x17 - end of section

This separation is necessary for correct reading of the transaction on the server side
The client sequentially writes the transaction data to the socket of TCP connection with the server

After the data transfer, the client can close the socket on the client side without waiting for a response or leave the socket open to receive the result of transaction processing from the server.
The server will return the result by the client's session ID in either case.
Then the server performs asynchronous processing of the transaction and returns the result to the client.

### Usage Scenarios
The main use cases of the protocol:

1. Database transaction processing
   * Sending data modification requests (INSERT, UPDATE, DELETE)
   * Sending data reading requests (SELECT)
   * Transmit arbitrary scripts and stored procedures
   * Use as a gateway for queuing and routing database queries

2. Asynchronous task processing
   * Passing tasks for execution to worker processes
   * Sending messages to task/message queues (RabbitMQ and analogs)

3. WebSocket server
   * Tunneling WebSocket connections via TCP
   * Realization of chats, notifications, event monitoring and other real-time functionality.

4. HTTP server
   * Routing and balancing HTTP requests in web applications
   * Increase scalability through asynchronous queueing

5. Other cases
   * Send push notifications and email newsletters
   * Logging events and metrics to data warehouse
   * Synchronization of state in distributed systems
   * Any tasks that require high performance of data recording

The scope of the protocol is quite wide.

### Data structures
1. Transaction.
   The basic data structure of the protocol. It contains the following fields:
   * req - string with the initial test of the request
   * head - object with request headers
   * body - string with transaction data (JSON, XML, CSV, etc.)
   * to - UUID identifier of client session for response

2. Request header (It can also contain arbitrary additional headers with metadata).
   Includes the following attributes:
   * db - name of the database or other repository
   * table - name of a table, collection, job queue, etc.
   * key - record identifier
   * type - data type (json, string, int, date, etc.).

3. Transaction queue (Stores a queue of unprocessed transactions. Accessed by a pool of thread handlers).
   Represents a HashMap containing:
   * Key: transaction identifier
   * Value: transaction object TX

4. Client Matching
   Another HashMap of the global variable CLIENTS:
   * Key: UUID session identifier.
   * Value: client connection socket

Stores open connections and mappings to session IDs.

### Procedures
1. Sending data by the client
   * The client forms the body of the request and specifies the necessary headers (transaction type, resource name, id).
   * Splits the transaction into 2 parts using special characters.
   * Sequentially sends parts of the transaction to the socket of TCP connection
   * Can close the connection from the client side without waiting for a response.

2. Receiving data on the server
   * Server reads incoming data stream from socket.
   * Divides into header and body by special characters.
   * Parses the header and creates the RequestHeaders structure
   * Forms a complete TX transaction object by adding the session id
   * Places TX in the Pool processing queue

3. Asynchronous transaction processing
   * The worker-threads pool cyclically checks the Pool queue for new TXs
   * Processes registered transactions one by one
   * Executes the transaction according to the specified request type
   * Returns the resulting data set

4. Returns the result to the client
   * Worker takes client session id from TX header
   * Sends the result to the client by session id
   * Removes the completed transaction from the queue
   * Deletes the completed transaction from the queue

### Connection management
1. Connection establishment
   * Initiated by the client upon request to connect to the server via TCP
   * Server accepts incoming connection
   * UUID of the new session identifier is generated.
   * The identifier and socket are stored in a global variable.

In the established connection, transactions from the client and responses with processing results from the server are transmitted by identifiers.

2. Closing a connection
   * The client can initiate closure by simply closing the TCP connection
   * The server also closes the connection and deletes the session when errors occur
   * For example, in case of a connection failure or data timeout.

Thus, a full cycle of interaction management between client and server within the protocol is supported.

### Extensibility
This protocol is designed from the outset with room for expansion to support large workloads and integration with various systems.

1. Scaling
   * Increase the number of parallel connections by using asynchronous model and queues.
   * Increase the number of worker threads to handle large transaction volumes
   * Ability to distribute the load across multiple physical servers

2. Expansion of functionality
   * Adding support for new database types by implementing additional handler modules
   * Implementation of bundling for third-party systems and task queues
   * Implementation of additional transaction routing logic

3. Security
   * Support for SSL / TLS based encryption
   * Integration with authentication and authorization systems (OAuth, JWT) (not included in the protocol but can be implemented additionally)
   * Transaction validation by schemes to protect against incorrect or malicious requests (not included in the protocol but can be implemented additionally).

4. Fault tolerance
   * Adding buffering of unsent transactions to protect against failures
   * Replication and synchronization of state between multiple server instances
   * Support for publish and subscribe protocols for guaranteed delivery of all transactions

### Security
The protocol has the following capabilities to ensure data protection and security:

1. Traffic encryption
   * Supports TLS to encrypt data in transmission between client and server
   * Use of digital certificates and PKI infrastructure
   * Protection against eavesdropping and spoofing of transactions

2. Request authentication
   * Validation of connection identifiers against global session store.
   * Integration with external authentication systems (OAuth, LDAP) (not included in the protocol but can be implemented additionally)
   * Verification of API keys for transaction source identification (not included in the protocol but can be implemented additionally)

3. Access authorization
   * Setting ACL rules for resources and protocol operations
   * Restricting access of different user groups

4. Data validation
   * Checking the structure of transactions by schemes
   * Filtering incoming requests for correctness

5. Action logging
   * Logging transactions with session IDs
   * Monitoring and notification of suspicious activity

This protocol can thus be adapted flexibly enough to meet the security requirements of specific systems.

