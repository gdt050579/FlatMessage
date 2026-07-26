output = '''
use flat_message::FlatMessage;
'''
n = 20

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
'''
    for j in range(0, i):
        x += f"    field_{j}: u32,\n"
    x += "}"
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