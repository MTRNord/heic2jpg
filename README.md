# HEIC2JPG

## Building the project

Make sure you have `flatpak` and `flatpak-builder` installed. Then run the commands below. Replace `<application_id>` with the value you entered during project creation. Please note that these commands are just for demonstration purposes. Normally this would be handled by your IDE, such as GNOME Builder or VS Code with the Flatpak extension.

```shell
flatpak install --user org.gnome.Sdk//46 org.gnome.Platform//46  org.freedesktop.Sdk.Extension.rust-stable//23.08 org.freedesktop.Sdk.Extension.llvm16//23.08
flatpak-builder --user flatpak_app build-aux/<application_id>.Devel.json
```

## Running the project

Once the project is build, run the command below. Replace Replace `<application_id>` and `<project_name>` with the values you entered during project creation. Please note that these commands are just for demonstration purposes. Normally this would be handled by your IDE, such as GNOME Builder or VS Code with the Flatpak extension.

```shell
flatpak-builder --run flatpak_app build-aux/<application_id>.Devel.json <project_name>
```

## Extract gettext strings

`xgettext --package-name=heic2jpg --package-version=main --msgid-bugs-address=https://github.com/MTRNord/heic2jpg/issues --files-from=po/POTFILES.in --output=po/heic2jpg.pot`
