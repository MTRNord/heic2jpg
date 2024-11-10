# HEIC2JPG

## About

HEIC2JPG is an application to batch convert HEIC images to JPG images. It is packaged as a Flatpak for easy installation and usage.

## Building the project

```shell
flatpak install --user org.gnome.Sdk//46 org.gnome.Platform//46  org.freedesktop.Sdk.Extension.rust-stable//23.08 org.freedesktop.Sdk.Extension.llvm16//23.08
flatpak-builder --user flatpak_app build-aux/dev.nordgedanken.heic2jpg.Devel.json
```

## Running the project

```shell
flatpak-builder --run flatpak_app build-aux/dev.nordgedanken.heic2jpg.Devel.json heic2jpg
```

## Installing the application

To install the application, add the repository and install the package using the following commands:

```shell
flatpak remote-add --user heic2jpg https://mtrnord.github.io/heic2jpg/index.flatpakrepo
flatpak install --user dev.nordgedanken.heic2jpg.Devel
```

## Translations

### Extract gettext strings

`xgettext --package-name=heic2jpg --package-version=main --msgid-bugs-address=https://github.com/MTRNord/heic2jpg/issues --files-from=po/POTFILES.in --output=po/heic2jpg.pot`

### Adding a new language and translating

To add a new language and translate the application, follow these steps:

1. **Add the language to the LINGUAS file**:
   Open the `po/LINGUAS` file and add the language code (e.g., `fr` for French) to the list.

2. **Generate the .po file for the new language**:
   Use the `msginit` command to create a new `.po` file for the language. For example, to add French translations, run:

   ```shell
   msginit --input=po/heic2jpg.pot --locale=fr --output-file=po/fr.po
   ```

3. **Translate the strings**:
   Open the newly created `po/fr.po` file in a text editor or a translation tool like Poedit and translate the strings.

By following these steps, you can add support for new languages and provide translations for the HEIC2JPG application.
