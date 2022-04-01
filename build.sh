
for dir in $(ls -1d */ | xargs -n1 basename); do
	pushd ${dir}
		cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-unknown-linux-gnu -r
	popd
done

for dir in $(ls -1d */ | xargs -n1 basename); do
    ls -alh ${dir}/target/x86_64-unknown-linux-gnu/release/${dir}
done
