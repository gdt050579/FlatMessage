
use flat_message::FlatMessage;

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated0 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated1 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated2 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated3 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated4 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated5 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated6 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated7 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated8 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated9 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated10 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated11 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated12 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated13 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated14 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated15 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated16 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated17 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated18 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated19 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated20 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated21 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated22 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated23 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated24 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated25 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated26 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated27 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated28 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated29 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated30 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated31 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated32 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated33 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated34 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated35 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated36 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated37 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated38 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated39 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated40 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated41 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated42 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated43 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated44 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated45 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated46 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated47 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated48 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated49 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated50 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated51 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated52 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated53 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated54 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated55 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated56 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated57 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated58 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated59 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated60 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated61 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated62 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated63 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated64 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated65 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated66 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated67 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated68 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated69 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated70 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated71 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated72 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated73 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated74 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated75 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated76 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated77 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated78 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated79 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated80 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated81 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated82 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated83 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated84 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated85 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated86 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated87 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated88 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated89 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated90 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated91 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated92 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated93 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated94 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated95 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated96 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated97 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated98 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated99 {
    // --- Existing Base Fields ---
    name: String,
    pid: u32,
    parent_pid: u32,
    parent: String,
    user: String,
    command_line: String,
    timestamp: flat_message::Timestamp,
    unique_id: flat_message::UniqueID,

    // --- Image & Path Details ---
    /// Full path to the executed binary
    image_path: String,
    /// The working directory when the process was spawned
    current_directory: String,
    /// Full path to the parent binary
    parent_image_path: String,
    /// The exact command line used to spawn the parent process
    parent_command_line: Option<String>,

    // --- Execution Context ---
    /// The thread ID of the main thread created
    main_thread_id: u32,
    /// The terminal/RDP session ID
    terminal_session_id: u32,
    /// Process creation flags (e.g., CREATE_SUSPENDED)
    creation_flags: u32,
    /// Environment variables passed to the new process
    environment_variables: Vec<String>,

    // --- Identity & Security (Windows/Linux) ---
    /// The Logon ID for correlating with authentication events
    logon_id: String,
    /// A unique GUID for the logon session
    logon_guid: Option<String>,
    /// The integrity level (e.g., "System", "High", "Medium", "Low")
    integrity_level: String,
    /// Security Identifier (SID) of the executing user
    sid: String,

    // --- Cryptographic Hashes ---
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    imphash: Option<String>,

    // --- Binary Metadata (PE/ELF details) ---
    /// Original filename from the binary's version resources
    original_file_name: Option<String>,
    /// Company name from the binary's version resources
    company: Option<String>,
    /// Description from the binary's version resources
    description: Option<String>,
    /// Product name from the binary's version resources
    product: Option<String>,
    /// File version string
    file_version: Option<String>,

    // --- Code Signing Information ---
    /// E.g., "Valid", "Invalid", "Unsigned", "Revoked"
    signature_status: Option<String>,
    /// The subject name of the certificate used to sign the binary
    signer: Option<String>,

    // --- Cloud/Container Context ---
    /// Docker/Kubernetes container ID if running in a containerized environment
    container_id: Option<String>,
}

fn main() {
    println!("Hello, world!");
}
