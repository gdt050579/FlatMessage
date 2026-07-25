output = '''
use flat_message::FlatMessage;
'''
n = 50

for i in range(0, n):
    x = f'''
#[derive(FlatMessage)]
#[flat_message_options(version: 1, optimized_unchecked_code=false)]
struct ProcessCreated{i} {{
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
}}
'''
    output += x

output += '''

fn main() {
    let storage = flat_message::Storage::from_buffer(b"agnn342jqtrk234nnsadknakfno");
    match std::env::args().count() {
'''

for i in range(0, n):
    output += f"        {i} => {{ ProcessCreated{i}::deserialize_from_ref(&storage).unwrap(); }}\n"

output += '''        _ => panic!()
}}'''

with open("tach/src/main.rs", "w") as f:
    f.write(output)