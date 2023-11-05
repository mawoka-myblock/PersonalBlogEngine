<script lang="ts">
    import ClassicEditor from "@ckeditor/ckeditor5-build-classic"

    import {upload_files} from "./upload_files";
    import {onMount} from "svelte";
    // import SimpleUploadAdapter from '@ckeditor/ckeditor5-upload/src/adapters/simpleuploadadapter';
    // import ImageResize from "@ckeditor/ckeditor5-image/src/imageresize"

    export let value = "";

    let div_editor: HTMLDivElement
    let editor


    class MyUploadAdapter {
        constructor(loader) {
            // The file loader instance to use during the upload.
            this.loader = loader;
        }

        /*        xhr
                loader*/

        _initListeners(resolve, reject, file) {
            const xhr = this.xhr;
            const loader = this.loader;
            const genericErrorText = `Couldn't upload file: ${file.name}.`;

            xhr.addEventListener('error', () => reject(genericErrorText));
            xhr.addEventListener('abort', () => reject());
            xhr.addEventListener('load', () => {
                const response = xhr.response;
                if (!response || response.error) {
                    return reject(response && response.error ? response.error.message : genericErrorText);
                }

                console.log(response)

                resolve({
                    default: `${window.location.origin}/api/v1/uploads/${response.id}`
                });
            });


            if (xhr.upload) {
                xhr.upload.addEventListener('progress', evt => {
                    if (evt.lengthComputable) {
                        loader.uploadTotal = evt.total;
                        loader.uploaded = evt.loaded;
                    }
                });
            }

        }

        _initRequest() {
            const xhr = this.xhr = new XMLHttpRequest();
            xhr.open('POST', '/api/v1/uploads/', true);
            xhr.responseType = 'json';
            xhr.setRequestHeader("Content-Type", "application/json")
        }

        _sendRequest(file) {
            this.xhr.send(JSON.stringify({
                file_name: "File_upload",
                // mime_type: file.type,
                data: this.loader._reader._data.replace("data:", "").replace(/^.+,/, "")
            }));
        }

        // Starts the upload process.
        upload() {
            // Update the loader's progress.
            // const b64_data = this.loader._reader._data.replace("data:", "").replace(/^.+,/, "")
            return this.loader.file
                .then(file => new Promise((resolve, reject) => {
                    this._initRequest();
                    this._initListeners(resolve, reject, file);
                    this._sendRequest(file);
                }));
        }

        // Aborts the upload process.
        abort() {
            // Reject the promise returned from the upload() method.
            if (this.xhr) {
                this.xhr.abort();
            }
        }
    }


    function

    MyCustomUploadAdapterPlugin(editor) {
        editor.plugins.get('FileRepository').createUploadAdapter = (loader) => {
            // Configure the URL to the upload script in your back-end here!
            return new MyUploadAdapter(loader);
        };
    }

    const
        upload_image = async (files: Array<File>) => {
            let promises = []
            for (const file of files) {
                promises.push(upload_files(file))
            }
            const res = await Promise.all(promises)
            console.log(res)
            return res
        }

    $: value = value.replace('<p>', '').replace('</p>', '');

    const
        triggerChange = () => {
            value = editor.getData();
            console.log("ValueChange!")
        };

    onMount(
        () => {
            // console.log(ClassicEditor)
            ClassicEditor
                .create(div_editor, {
                    extraPlugins: [MyCustomUploadAdapterPlugin],
                })

                .then(
                    (
                        e
                    ) => {
                        editor = e;
                        console.log(e)
                        editor
                            .setData(value);

                        e
                            .model
                            .document
                            .on(
                                "change:data"
                                , () => {
                                    triggerChange()
                                }
                            )
                    })
                .catch((e) => console.log(e))
        })
</script>

<div class="prose w-screen h-full">
    <div class="w-screen h-full">
        <div bind:this={div_editor} class="h-full w-full"></div>
    </div>
</div>
