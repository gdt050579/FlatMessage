import os

global_speed = {}
def process_file(file_path):
    global global_speed
    exec_name = ""
    in_table = False
    data_size = 0
    n = 0
    for line in open(file_path,"rt", encoding="utf-8"):
        if line.startswith("### 1. Windows Execution"):
            exec_name = "win"
        elif line.startswith("### 2. MacOs Execution"):
            exec_name = "mac"
        elif line.startswith("### 3. Linux Execution"):
            exec_name = "linux"
        if line.startswith("* Data size: "):
            data_size = int(line.strip().replace("* Data size: ","").replace("bytes","").replace("`","").strip())
        if line.startswith("* Serialization"):
            n = int(line.split("`n = ",1)[1].replace("`","").strip())


        if line.startswith("| Algorithm |"):
            in_table = True
            continue
        if line.strip() == "":
            in_table = False
            continue
        if not in_table:
            continue
        if line.startswith("|"):
            w = line.strip().split("|")
            des_name = w[1].strip()
            name = des_name.split(" ",1)[0].strip()
            if name.startswith("-"): continue
            time = w[5].strip().split("<",1)[0].replace("*","").strip()
            ts = data_size * n / 1000000
            if not time.startswith("-"):
                st = float(time.strip())
                speed = ts * 1000.0 / st
            else:
                speed = "-"
            # print(exec_name, name, time, data_size, n, "speed = ",speed, " MB/sec")
            if not exec_name in global_speed:
                global_speed[exec_name] = {}
            if not des_name in global_speed[exec_name]:
                global_speed[exec_name][des_name] = []
            global_speed[exec_name][des_name] += [speed]

for name in os.listdir("../book/chapter-5/results"):
    process_file(os.path.join("../book/chapter-5/results", name))

# build
d = {}
l = []
for exec_name in global_speed:
    for des_name in global_speed[exec_name]:
        if des_name == "protobuf":
            continue
        if not des_name in d:
            d[des_name] = len(l)
            l += [{"name": des_name, "win": 0, "mac": 0, "linux": 0}]
        s = 0
        for speed in global_speed[exec_name][des_name]:
            s += float(speed)
        s = s / len(global_speed[exec_name][des_name])
        if exec_name == "win":
            l[d[des_name]]["win"] = s
        elif exec_name == "mac":
            l[d[des_name]]["mac"] = s
        elif exec_name == "linux":
            l[d[des_name]]["linux"] = s

l.sort(key=lambda x: x["win"], reverse=True)

print("| Algorithm | Win (MB/sec)| Mac (MB/sec)| Linux (MB/sec)|")
print("| ------ | -------: | -------: | -------: |")
for i in l:
    print(f"| {i['name']} | {i['win']:.2f} | {i['mac']:.2f} | {i['linux']:.2f} |")