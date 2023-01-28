#!/usr/bin/env bash
    killall qemu-system-riscv64 # 由于无法在debug结束时关闭虚拟机，我们在debug开始时关闭上一次开启的虚拟机。
    nohup bash -c "make gdbserver > run.log 2>&1" & # 后台启动qemu
    echo "Done!"1