rm -r ./client-ts/anchor-artifacts/*
cp -r ./anchor/target/idl ./client-ts/anchor-artifacts/
cp -r ./anchor/target/types ./client-ts/anchor-artifacts/

rm -r ./frontend/anchor-artifacts/*
cp -r ./anchor/target/idl ./frontend/anchor-artifacts/
cp -r ./anchor/target/types ./frontend/anchor-artifacts/
