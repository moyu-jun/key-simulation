"""
pynput 按键模拟测试脚本
功能与 Rust 项目一致：F10 切换启停，每 1000ms 模拟一次 Z 键

需要管理员权限运行（部分游戏需要）
依赖：pip install pynput
"""

import ctypes
import sys
import time
import threading
from pynput.keyboard import Controller, Key, KeyCode, Listener


def is_admin():
    try:
        return ctypes.windll.shell32.IsUserAnAdmin() != 0
    except Exception:
        return False


def ensure_admin():
    if is_admin():
        return
    print("未以管理员身份运行，正在尝试提权...")
    params = " ".join(f'"{a}"' for a in sys.argv)
    ret = ctypes.windll.shell32.ShellExecuteW(
        None, "runas", sys.executable, params, None, 1
    )
    if ret <= 32:
        print("提权失败，请手动以管理员身份运行")
        sys.exit(1)
    sys.exit(0)


class KeySimulator:
    def __init__(self, mode: str):
        """
        mode:
            'key'  - 使用 KeyCode.from_vk(0x5A) 路径（虚拟键码）
            'char' - 使用 'z' 字符路径（UNICODE 模式）
        """
        self.mode = mode
        self.kbd = Controller()
        self.active = False
        self.stop = False

    def press_once(self):
        if self.mode == "key":
            target = KeyCode.from_vk(0x5A)
        else:
            target = "z"
        self.kbd.press(target)
        time.sleep(0.05)
        self.kbd.release(target)

    def loop(self):
        while not self.stop:
            if self.active:
                try:
                    self.press_once()
                except Exception as e:
                    print(f"模拟失败: {e}")
            time.sleep(1.0)

    def on_press(self, key):
        if key == Key.f10:
            self.active = not self.active
            state = "启动" if self.active else "停止"
            print(f"[{state}] 模拟已{'开启' if self.active else '关闭'}")
        elif key == Key.esc:
            print("退出程序")
            self.stop = True
            return False


def main():
    ensure_admin()

    mode = "key"
    if "--mode" in sys.argv:
        idx = sys.argv.index("--mode")
        if idx + 1 < len(sys.argv):
            mode = sys.argv[idx + 1]

    if mode not in ("key", "char"):
        print(f"未知模式: {mode}，可选: key | char")
        sys.exit(1)

    sim = KeySimulator(mode)

    print("=== pynput 按键模拟测试 ===")
    label = "press(KeyCode.from_vk(0x5A))" if mode == "key" else "press('z')"
    print(f"模式: {mode}  ({label})")
    print("按 F10 启动/停止模拟（每 1000ms 模拟 Z 键）")
    print("按 ESC 退出程序")

    worker = threading.Thread(target=sim.loop, daemon=True)
    worker.start()

    with Listener(on_press=sim.on_press) as listener:
        listener.join()


if __name__ == "__main__":
    main()
