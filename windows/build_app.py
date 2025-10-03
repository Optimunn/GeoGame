#!/Windows OS
import os
import sys
import platform
import subprocess
import shutil

APP_VERSION = "0.6.0"
APP_NAME = "Geographical game"
BIN_NAME = "geo_game"
OUTPUT_DIR = "."

BIN_EXT = ".exe"
BINARY_PATH = os.path.join("..", "target", "release", BIN_NAME + BIN_EXT)

if platform.system() != "Windows":
    print("\033[31mError: This script is only for Windows OS!\033[0m")
    sys.exit(1)

def cargo_check():
    try:
        subprocess.check_output(["cargo", "--version"], stderr=subprocess.STDOUT)
        print("Cargo OK.")
    except:
        print("Error: Cargo is not installed!")
        sys.exit(1)

APP_DIR = os.path.join(OUTPUT_DIR, APP_NAME)

if len(sys.argv) > 1:
    arg = sys.argv[1]
    if arg in ["--clear", "-c"]:
        if os.path.exists(APP_DIR):
            print("Deleting previous version...")
            shutil.rmtree(APP_DIR)
        sys.exit(0)
    elif arg in ["--help", "-h"]:
        print("Options:")
        print(" --clear, -c Clear previous version")
        print(" --help, -h Print this help message")
        print(" --release Build release binary")
        print(" --all Build release application")
        print(" (no options) Build app and activate it")
        sys.exit(0)
    elif arg == "--release":
        cargo_check()
        print("Build release binary...")
        try:
            subprocess.check_call(["cargo", "build", "--release"])
        except:
            sys.exit(1)
        sys.exit(0)
    elif arg == "--all":
        cargo_check()
        print("Build release application...")
        try:
            subprocess.check_call(["cargo", "build", "--release"])
        except:
            sys.exit(1)
    else:
        print("Unknown option:", arg)
        print("Use --help or -h for help")
        sys.exit(1)

def check_file_exists(path):
    if not os.path.isfile(path):
        print("Error: File not found:", path)
        sys.exit(1)

check_file_exists(BINARY_PATH)

CONTENTS_DIR = APP_DIR
APP_BIN_NAME = APP_NAME + BIN_EXT
APP_BIN_PATH = os.path.join(CONTENTS_DIR, APP_BIN_NAME)

if os.path.exists(APP_BIN_PATH):
    print("Changing binary file...")
    shutil.copy(BINARY_PATH, APP_BIN_PATH)
    print("Application successfully recreated:", APP_DIR)
    sys.exit(0)

if os.path.exists(APP_DIR):
    print("Deleting previous version...")
    shutil.rmtree(APP_DIR)

RESOURCES_DIR_DATA = os.path.join(CONTENTS_DIR, "Data")
ASSETS_DIR = os.path.join(CONTENTS_DIR, "Assets")
ASSETS_DIR_FLAGS = os.path.join(ASSETS_DIR, "flags")
ASSETS_DIR_ICONS = os.path.join(ASSETS_DIR, "icons")

print("Creating directory structure...")
os.makedirs(RESOURCES_DIR_DATA)
os.makedirs(ASSETS_DIR)
os.makedirs(ASSETS_DIR_FLAGS)
os.makedirs(ASSETS_DIR_ICONS)

IMG_PATH = "..\\assets\\flags\\4x3"
ICS_PATH = "..\\assets\\icons"
DATA_PATH = "..\\data"

print("Copying files...")
shutil.copy(BINARY_PATH, APP_BIN_PATH)

# Copy contents of directories
for item in os.listdir(IMG_PATH):
    src_item = os.path.join(IMG_PATH, item)
    if os.path.isdir(src_item):
        shutil.copytree(src_item, os.path.join(ASSETS_DIR_FLAGS, item))
    else:
        shutil.copy(src_item, ASSETS_DIR_FLAGS)

for item in os.listdir(ICS_PATH):
    src_item = os.path.join(ICS_PATH, item)
    if os.path.isdir(src_item):
        shutil.copytree(src_item, os.path.join(ASSETS_DIR_ICONS, item))
    else:
        shutil.copy(src_item, ASSETS_DIR_ICONS)

for item in os.listdir(DATA_PATH):
    src_item = os.path.join(DATA_PATH, item)
    if os.path.isdir(src_item):
        shutil.copytree(src_item, os.path.join(RESOURCES_DIR_DATA, item))
    else:
        shutil.copy(src_item, RESOURCES_DIR_DATA)

print("Application successfully created:", APP_DIR)