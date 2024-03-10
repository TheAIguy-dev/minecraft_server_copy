#!/bin/bash

function pprint {
    awk '
        /total/ {
            printf $1;
            for(c=0;c<32-length($2)-length($1);c++) printf " ";
            print $2;
        }
        NR > 1 {
            printf $2;
            for(c=0;c<32-length($2)-length($1);c++) printf " ";
            print $1;
        }
    ' $*
}

cargo fmt

case "$1" in
    d | debug | "")
        cargo run "${@:2}"
        ;;
    r | release)
        read -p "What to name the release? (default Test) " NAME
        if [ "$NAME" == "" ]; then
            NAME="Test"
        fi

        echo "What to optimize for?"
        echo "1. Speed (default)"
        echo "2. Size"
        echo "3. Both"
        read -p "> " PROFILE
        if [ "$PROFILE" != "1" ] && [ "$PROFILE" != "2" ] && [ "$PROFILE" != "3" ]; then
            PROFILE=1
        fi

        rm -rf "./releases/$NAME"
        mkdir "./releases/$NAME"

        echo "Starting build. Please wait..."
        start=`date +%s.%N`

        # Optimize for speed
        if [ "$PROFILE" == "1" ] || [ "$PROFILE" == "3" ]; then
            mkdir "./releases/$NAME/Speed"

            cross build -q --target x86_64-pc-windows-gnu --profile release
            mv "./target/x86_64-pc-windows-gnu/release/minecraft_protocol.exe" "./releases/$NAME/Speed/windows.exe"
            cargo build -q --target x86_64-unknown-linux-gnu --profile release
            mv "./target/x86_64-unknown-linux-gnu/release/minecraft_protocol" "./releases/$NAME/Speed/linux"

            echo
            echo "============ SPEEDY ============"
            ls -sh "./releases/$NAME/Speed/" | pprint
            echo "================================"
        fi
        # Optimize for size
        if [ "$PROFILE" == "2" ] || [ "$PROFILE" == "3" ]; then
            mkdir "./releases/$NAME/Size"

            cross build -q --target x86_64-pc-windows-gnu --profile size
            mv "./target/x86_64-pc-windows-gnu/size/minecraft_protocol.exe" "./releases/$NAME/Size/windows.exe"
            cargo build -q --target x86_64-unknown-linux-gnu --profile size
            mv "./target/x86_64-unknown-linux-gnu/size/minecraft_protocol" "./releases/$NAME/Size/linux"

            echo
            echo "============= SIZE ============="
            ls -sh "./releases/$NAME/Size/" | pprint
            echo "================================"

            cp -r "./releases/$NAME/Size" "./releases/$NAME/Size (agressive)"
            upx --best -q "./releases/$NAME/Size (agressive)/linux" "./releases/$NAME/Size (agressive)/windows.exe" > "/dev/null"

            echo
            echo "======= SIZE (AGRESSIVE) ======="
            ls -sh "./releases/$NAME/Size (agressive)/" | pprint
            echo "================================"

        fi

        end=`date +%s.%N`
        echo
        echo "Build finished in $(echo "$end - $start" | bc -l)s"
        ;;
    *)
        echo "Invalid argument"
        exit 1
esac

exit 0
