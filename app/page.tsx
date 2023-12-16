"use client";
import { Input } from "@nextui-org/react";
import { ChangeEvent, useState, useEffect } from "react";
import toast from "react-hot-toast";
import { useHotkeys } from "react-hotkeys-hook";

export default function Home() {
  const [phone, setPhone] = useState<string>("");
  const [limit, setLimit] = useState<string>("");
  const [page, setPage] = useState<string>("USSD");

  const handlePhoneChange = (e: ChangeEvent<HTMLInputElement>): void => {
    setPhone(e.target.value);
  };

  const handleLimitChange = (e: ChangeEvent<HTMLInputElement>): void => {
    setLimit(e.target.value);
  };

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>): void => {
    e.preventDefault();
    if (!phone || !limit) {
      toast.error("Phone and Limit are required");
      return;
    }
    toast.success("Form submitted!");
    console.log({ phone, limit });
    setPhone("");
    setLimit("");
  };
  let isEnabled = false;

  useEffect(() => {
    function handleContextMenu(e: MouseEvent) {
      e.preventDefault();
    }
    document.addEventListener("contextmenu", handleContextMenu);

    return () => {
      document.removeEventListener("contextmenu", handleContextMenu);
    };
  }, []);
  useHotkeys(
    ["ctrl+u", "ctrl+shift+a+c", "ctrl+shift+i"],
    () => {
      toast.error("Hot keys are disabled");
    },
    {
      enabled: () => false,
      preventDefault: true,
    }
  );

  return (
    <main className="w-full h-full bg-gray-900 flex items-center justify-center">
      <section className="h-full w-full bg-gray-900 ">
        <div className="h-10 bg-gray-800 flex justify-between">
          <h1 className="text-sm mx-5 h-2 my-2.5">USSD Mapper</h1>
          <div className="mr-5">
            <button
              className="mx-2 h-4  hover:bg-gray-400"
              onClick={() => setPage("USSD")}
            >
              USSD
            </button>
            <button
              className="mx-2 h-2 my-2 hover:bg-gray-400"
              onClick={() => setPage("SMS")}
            >
              SMS
            </button>
          </div>
        </div>
        {/* USSD page */}
        <div
          className={`${
            page === "USSD" ? "flex" : "hidden"
          } flex-col mt-1 bg-gray-800`}
          style={{ height: "calc(100vh - 45px)" }}
        >
          <h1 className="align-start justify-start text-xl font-bold mt-5 ml-5">
            USSD
          </h1>
          <form onSubmit={handleSubmit}>
            <div className="flex flex-col justify-center items-center h-96">
              <input
                type="number"
                placeholder="Type number"
                value={phone}
                onChange={handlePhoneChange}
                className="px-2 h-10 w-56 rounded-xl my-2 appearance-none bg-gray-600 hover:bg-gray-700"
                style={{
                  WebkitAppearance: "none",
                  MozAppearance: "textfield",
                }}
                required
              />
              <input
                type="number"
                placeholder="Type limit"
                value={limit}
                onChange={handleLimitChange}
                className="px-2 h-10 w-56 rounded-xl my-2 appearance-none bg-gray-600 hover:bg-gray-700"
                style={{
                  WebkitAppearance: "none",
                  MozAppearance: "textfield",
                }}
                required
              />
              <button
                type="submit"
                className="px-2 h-10 w-56 rounded-xl my-2 bg-gray-600 hover:bg-gray-700"
              >
                Start
              </button>
            </div>
          </form>
        </div>
        {/* SMS page */}
        <div
          className={`${
            page === "SMS" ? "flex" : "hidden"
          } flex-col mt-1 bg-gray-800`}
          style={{ height: "calc(100vh - 45px)" }}
        >
          <h1 className="align-start justify-start text-xl font-bold mt-5 ml-5">
            SMS
          </h1>
          <div className="flex flex-col justify-center items-center">
            <div className="flex flex-col h-96 w-96 mt-10 gap-2 bg-gray-900 rounded-lg p-2 overflow-y-auto">
              <p className="text-sm bg-gray-600 p-2 rounded-lg">
                Lorem ipsum dolor sit amet consectetur adipisicing elit. Quos
                eos, deleniti neque necessitatibus inventore perspiciatis
                pariatur in ad cupiditate corrupti consequatur blanditiis,
              </p>
              <p className="text-sm bg-gray-600 p-2 rounded-lg">
                Lorem ipsum dolor sit amet consectetur adipisicing elit. Quos
                eos, deleniti neque necessitatibus inventore perspiciatis
                pariatur in ad cupiditate corrupti consequatur blanditiis,
              </p>
              <p className="text-sm bg-gray-600 p-2 rounded-lg">
                Lorem ipsum dolor sit amet consectetur adipisicing elit. Quos
                eos, deleniti neque necessitatibus inventore perspiciatis
                pariatur in ad cupiditate corrupti consequatur blanditiis,
              </p>
              <p className="text-sm bg-gray-600 p-2 rounded-lg">
                Lorem ipsum dolor sit amet consectetur adipisicing elit. Quos
                eos, deleniti neque necessitatibus inventore perspiciatis
                pariatur in ad cupiditate corrupti consequatur blanditiis,
              </p>
              <p className="text-sm bg-gray-600 p-2 rounded-lg">
                Lorem ipsum dolor sit amet consectetur adipisicing elit. Quos
                eos, deleniti neque necessitatibus inventore perspiciatis
                pariatur in ad cupiditate corrupti consequatur blanditiis,
              </p>
              <p className="text-sm bg-gray-600 p-2 rounded-lg">
                Lorem ipsum dolor sit amet consectetur adipisicing elit. Quos
                eos, deleniti neque necessitatibus inventore perspiciatis
                pariatur in ad cupiditate corrupti consequatur blanditiis,
              </p>
              <p className="text-sm bg-gray-600 p-2 rounded-lg">
                Lorem ipsum dolor sit amet consectetur adipisicing elit. Quos
                eos, deleniti neque necessitatibus inventore perspiciatis
                pariatur in ad cupiditate corrupti consequatur blanditiis,
              </p>
              <p className="text-sm bg-gray-600 p-2 rounded-lg">
                Lorem ipsum dolor sit amet consectetur adipisicing elit. Quos
                eos, deleniti neque necessitatibus inventore perspiciatis
                pariatur in ad cupiditate corrupti consequatur blanditiis,
              </p>
            </div>
          </div>
        </div>
      </section>
    </main>
  );
}
