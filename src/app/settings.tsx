export default function Settings({
    settings,
    handleChange,
    toggleDialog
  }: any) {
    return (
      <>
        {/* ----->  Taken from Tailwind CSS example: https://tailwindui.com/components/application-ui/overlays/dialogs */}
        <div className="fixed inset-0 z-10 w-screen overflow-y-auto">
          <div className="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
            {/* <-----  */}
            <div className="border border-black shadow-2xl bg-white p-10">
              <h1 className="font-bold text-4xl text-center mt-10">Settings</h1>
              <div>
                <h2 className="font-bold pr-5 mb-10 border-b-2 border-b-gray-900 w-fit pb-2 text-2xl">
                External Utilities Settings
                </h2>
                <div>
                <div className="flex flex-row space-x-5">
                    <div className="mb-4">
                    <label className="font-bold block mb-2">
                        Radiance Path (binary)
                    </label>
                    <input
                        name="radiancePath"
                        type="text"
                        value={settings.radiancePath}
                        onChange={handleChange}
                        className="placeholder:text-right no-spinner w-40 shadow appearance-none border border-gray-400 rounded py-2 px-3 leading-tight focus:outline-none focus:shadow-outline"
                    ></input>
                    </div>
                    <div className="mb-4">
                    <label className="font-bold block mb-2">hdrgen Path</label>
                    <input
                        name="hdrgenPath"
                        type="text"
                        value={settings.hdrgenPath}
                        onChange={handleChange}
                        className="placeholder:text-right no-spinner w-40 shadow appearance-none border border-gray-400 rounded py-2 px-3 leading-tight focus:outline-none focus:shadow-outline"
                    ></input>
                    </div>
                    <div className="mb-4">
                    <label className="font-bold block mb-2">Output Path</label>
                    <input
                        name="outputPath"
                        type="text"
                        value={settings.outputPath}
                        onChange={handleChange}
                        className="placeholder:text-right no-spinner w-40 shadow appearance-none border border-gray-400 rounded py-2 px-3 leading-tight focus:outline-none focus:shadow-outline"
                    ></input>
                    </div>
                    <div className="mb-4">
                    <label className="font-bold block mb-2">
                        Temp Path (debug only)
                    </label>
                    <input
                        name="tempPath"
                        type="text"
                        value={settings.tempPath}
                        onChange={handleChange}
                        className="placeholder:text-right no-spinner w-40 shadow appearance-none border border-gray-400 rounded py-2 px-3 leading-tight focus:outline-none focus:shadow-outline"
                    ></input>
                    </div>
                </div>
                </div>
                <h2 className="font-bold pr-5 mb-10 border-b-gray-900 border-b-2 w-fit pb-2 text-2xl">
                App Settings
                </h2>
                <div>
                {/* <div className="flex flex-row space-x-5 items-center">
                    <label className="block mb-2">Show Image Previews</label>
                    <input type="checkbox" defaultChecked={true}></input>
                </div> */}
                </div>
                <div>
                  <button
                    type="button"
                    className="bg-gray-300 hover:bg-gray-400 text-gray-700 font-semibold py-1 px-2 border-gray-400 rounded"
                    onClick={toggleDialog}
                  >
                    Close
                  </button>
                  <div className="pt-2"></div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </>
    );
  }
  