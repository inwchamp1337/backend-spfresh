cd ./SPFresh
mkdir build && cd build
cmake -DCMAKE_BUILD_TYPE=Release ..
make -j$(nproc)

# สร้างไฟล์ release-artifacts.txt
find . -type f \( -perm /111 -o -name '*.so' -o -name '*.so.*' -o -name '*.a' -o -name '*.dylib' -o -name '*.dll' \) ! -name '*.o' ! -name '*.cmake' -print | sed 's|^./||' | sort > release-artifacts.txt || true
cat release-artifacts.txt

# บันทึก output จาก paths ใน /SPFresh/build และ /SPFresh/Release
echo ""
echo "=== Details of artifacts ==="
while read -r path; do
  [ -z "$path" ] && continue
  full_path="/SPFresh/build/$path"
  if [ -f "$full_path" ]; then
    echo "File: $full_path"
    ls -la "$full_path"
    file "$full_path" 2>/dev/null || echo "Type: unknown"
  elif [ -d "$full_path" ]; then
    echo "Directory: $full_path"
    ls -la "$full_path" | head -10
  else
    echo "Not found in build: $full_path"
  fi

  # Check in /SPFresh/Release
  release_path="/SPFresh/Release/$path"
  if [ -f "$release_path" ]; then
    echo "Also in Release: $release_path"
    ls -la "$release_path"
  elif [ -d "$release_path" ]; then
    echo "Also in Release: $release_path"
    ls -la "$release_path" | head -10
  fi
  echo ""
done < release-artifacts.txt

# ถ้าอยากเปลี่ยนชื่อไฟล์เป็น 'release'
# mv release-artifacts.txt release
# cat release