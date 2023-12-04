declare prgRoot="$(pwd)"
declare rootDocker="${prgRoot}/docker"
declare location="$rootDocker/build"
declare dockerImgs="ifscraucncinterface"
declare filterArch="dockerfile"

listArchs() {
    echo $(ls -lha $rootDocker | awk '{print $9}' | grep -v "\.dev\$" | grep "$filterArch")
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
            folderArch=$(pathArch $arch)
            echo "Creating $dockerImgs/$arch"
            mkdir -p "$folderArch"
            cp "$rootDocker/$arch" "$folderArch"
            cd "$folderArch"
            if [ `docker image ls | grep -c "$dockerImgs/$arch"` -eq 0 ] ;then
                docker build . -t "$dockerImgs/$arch" -f "$arch";
            fi
            echo "Image created $dockerImgs/$arch";
        done
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
    dev)
        devImage="$dockerImgs/dev"
        if [ `docker image ls | grep -c "$devImage"` -eq 0 ] ;then
            docker build . -t "$devImage" -f "$rootDocker/dockerfile.dev";
        fi
        echo "Image created $devImage";
        docker run \
        -e DISPLAY=$DISPLAY \
        -v /tmp/.X11-unix:/tmp/.X11-unix/:rw \
        --ipc=host \
        -v $(pwd):/app \
        -it \
        --net=host \
        "$devImage"
    ;;
    *)
        echo wrong option
    ;;
esac
