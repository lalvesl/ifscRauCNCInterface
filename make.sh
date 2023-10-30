declare prgRoot="$(pwd)"
declare rootDocker="${prgRoot}/docker"
declare location="$rootDocker/build"
declare dockerImgs="ifscraucncinterface"
declare filterArch="dockerfile"

listArchs() {
    echo $(ls -lha $rootDocker | awk '{print $9}' | grep "$filterArch")
}

getArch(){
    echo $1 | sed "s/$filterArch//g" | sed "s/\\.//g"
}

pathArch(){
    echo "$rootDocker/build_$(getArch $1)"
}

case $1 in
    init)
        for arch in $(listArchs); do
            cat "$rootDocker/$arch"
            folderArch=$(pathArch $arch)
            echo $folderArch
            mkdir -p "$folderArch"
            cp "$rootDocker/$arch" "$folderArch"
            cd "$folderArch"
            docker build . -t "$dockerImgs/$arch" -f "$arch"
            #docker run --rm -ti -v $location/app "$dockerImgs/$arch"
        done
    ;;
    buildFrontEnd)
        trunk build
    ;;
    buildArchs)
        rm -rf "$location"
        mkdir -p "$location"
        cp -r src-tauri "$location"
        cp -r .cargo "$location"
        cp -r public "$location"
        cp -r src "$location"
        cp Cargo* "$location"
        cp index.html "$location"
        cp styles.css "$location"
        cp Trunk.toml "$location"
        for arch in $(listArchs); do
            echo $arch
            cd "$prgRoot"
            folderArch="$(pathArch $arch)"
            rm -rf $folderArch
            cp -r "$location" "$folderArch"
            cp "$rootDocker/$arch" "$folderArch"
            cd "$folderArch"
            docker run --rm -ti -v "$folderArch:/app" "$dockerImgs/$arch" \
            bash -c "ls -lha && trunk build && cargo tauri build --target armv7-unknown-linux-gnueabihf"
        done
    ;;
    *)
        echo wrong option
    ;;
esac
