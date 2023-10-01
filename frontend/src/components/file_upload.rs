impl Component for FileUpload {
    // ...
    fn view(&self) -> Html {
        html! {
            <div class="flex flex-col items-center justify-center min-h-screen bg-gray-100">
                <div class="p-8 bg-white rounded shadow-md w-96">
                    <h2 class="text-2xl font-semibold mb-4">{"Upload your resume"}</h2>
                    <div class="flex flex-col space-y-4">
                        <input
                            class="p-2 border border-gray-300 rounded"
                            type="file"
                            onchange=self.link.callback(|...| ...)
                        />
                        <button
                            class="p-2 bg-blue-500 text-white rounded hover:bg-blue-600 focus:outline-none focus:border-blue-700 focus:ring focus:ring-blue-200 active:bg-blue-700"
                            onclick=self.link.callback(|_| Msg::Upload)
                        >
                            {"Analyze Resume"}
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}
