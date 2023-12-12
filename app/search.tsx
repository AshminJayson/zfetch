"use client";

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

export default function Search() {
    const [searchValue, setSearchValue] = useState("");
    const [searchKeyword, setSearchKeyword] = useState("");

    useEffect(() => {
        if (searchKeyword == "") {
            setSearchValue("");
            return;
        }

        invoke<string>("matcher", { key: searchKeyword })
            .then((result) => setSearchValue(result))
            .catch(console.error);
    }, [searchKeyword]);

    // Necessary because we will have to use Greet as a component later.
    return (
        <div className="text-sm flex flex-col gap-4">
            <div className="relative h-11 w-full min-w-[200px]">
                <input
                    className="peer h-full w-full border-b border-blue-gray-200 bg-transparent pt-4 pb-1.5 font-sans text-sm font-normal text-blue-gray-700 outline outline-0 transition-all placeholder-shown:border-blue-gray-200 focus:border-gray-500 focus:outline-0 disabled:border-0 disabled:bg-blue-gray-50"
                    placeholder=" "
                    onChange={(e) => setSearchKeyword(e.target.value)}
                />
                <label className="after:content[''] pointer-events-none absolute left-0  -top-1.5 flex h-full w-full select-none !overflow-visible truncate text-[11px] font-normal leading-tight text-gray-500 transition-all after:absolute after:-bottom-1.5 after:block after:w-full after:scale-x-0 after:border-b-2 after:border-gray-500 after:transition-transform after:duration-300 peer-placeholder-shown:text-sm peer-placeholder-shown:leading-[4.25] peer-placeholder-shown:text-blue-gray-500 peer-focus:text-[11px] peer-focus:leading-tight peer-focus:text-gray-400 peer-focus:after:scale-x-100 peer-focus:after:border-gray-400 peer-disabled:text-transparent peer-disabled:peer-placeholder-shown:text-blue-gray-500">
                    Search
                </label>
            </div>
            {searchValue}
        </div>
    );
}
