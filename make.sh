rootDocker="docker"
location="$rootDocker/build"
dockerImgs="ifscraucncinterface"

listArchs() {
    echo $(ls -lha $rootDocker | awk '{print $9}' | grep dockerfile)
}

case $1 in
init)
    for arch in $(listArchs); do
        cat "$rootDocker/$arch"
        docker build . -t "$dockerImgs/$arch" -f "$rootDocker/$arch"
        docker run --rm -ti -v $location/app "$dockerImgs/$arch"
    done
    ;;
buildFrontEnd)
    trunk build
    ;;
buildArchs)
    rm -rf $location
    mkdir -p $location
    cp -r dist $location
    cp -r src-tauri $location
    cp -r .cargo $location
    cp -r public $location
    cp -r src $location
    cp Cargo* $location
    for arch in $(listArchs); do
        echo $arch
        docker run --rm -ti -v $(pwd):/app "$dockerImgs/$arch" \
        bash -c "ls -lha && trunk build && cargo tauri build --target armv7-unknown-linux-gnueabihf"
    done
    ;;
*)
    echo wrong option
    ;;
esac
