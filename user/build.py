'''
列出 src/bin 下的用户程序源文件，按顺序链接: cargo build --release --bin <filename>
每次链接的时候改一下 BASE_ADDRESS
'''
import os
import sys

base_address = 0x8040_0000
step = 0x200_000
linker = 'src/linker.ld'
mode = sys.argv[1]

app_id = 0
apps = os.listdir('src/bin')
apps.sort()
for app in apps:
    app = app[:app.find('.')]
    lines = []
    lines_before = []
    with open(linker, 'r') as f:
        for line in f.readlines():
            lines_before.append(line)
            line = line.replace(hex(base_address),
                                hex(base_address+step*app_id))
            lines.append(line)
    with open(linker, 'w+') as f:
        f.writelines(lines)
    if mode == "debug":
        os.system('cargo build --bin %s' % app)
    else:
        os.system('cargo build --bin %s --release' % app)
    print('[build.py] application %s start with address %s, size = %sMB' %
          (app, hex(base_address+step*app_id), step/1024/1024))
    with open(linker, 'w+') as f:
        f.writelines(lines_before)
    app_id = app_id + 1
