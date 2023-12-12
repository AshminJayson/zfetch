"use client";

import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

export default function Search() {
    const [searchResult, setSearchResult] = useState<string[][]>([]);
    const [searchKeyword, setSearchKeyword] = useState("");
    const [selectedResult, setSelectedResult] = useState(0);
    const keyInputRef = useRef<HTMLInputElement | null>(null);
    const valueInputRef = useRef<HTMLInputElement | null>(null);

    useEffect(() => {
        if (searchKeyword == "") {
            setSearchResult([]);
            return;
        }

        setSelectedResult(0);

        invoke<string[][]>("matcher", { key: searchKeyword })
            .then((result) => setSearchResult(result))
            .catch(console.error);
    }, [searchKeyword]);

    useEffect(() => {
        const keydownHandler = (e: KeyboardEvent) => {
            switch (e.key) {
                case "Enter":
                    if (searchResult.length > 0) {
                        console.log(searchResult[selectedResult]);
                        navigator.clipboard.writeText(
                            searchResult[selectedResult][1]
                        );
                    }
                    break;
                case "ArrowUp":
                    if (selectedResult > 0) {
                        setSelectedResult(selectedResult - 1);
                    }
                    break;
                case "ArrowDown":
                    if (selectedResult < searchResult.length - 1) {
                        setSelectedResult(selectedResult + 1);
                    }
                    break;
                default:
                    break;
            }
        };

        window.addEventListener("keydown", keydownHandler);
        return () => {
            window.removeEventListener("keydown", keydownHandler);
        };
    }, [searchResult, selectedResult]);

    const addRecord = () => {
        if (!keyInputRef.current || !valueInputRef.current) return;

        console.log(keyInputRef.current.value, valueInputRef.current.value);

        invoke("addrecord", {
            key: keyInputRef.current.value,
            value: valueInputRef.current.value,
        })
            .then(() => {
                keyInputRef.current!.value = "";
                valueInputRef.current!.value = "";
            })
            .catch(console.error);
    };

    return (
        <div className="relative text-sm flex flex-col gap-4 mb-8">
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
            <div className="mb-10 z-10">
                {searchResult.length > 0 &&
                    searchResult.map((result, ind) => (
                        <p
                            key={ind}
                            onClick={() => {
                                setSelectedResult(ind);
                                navigator.clipboard.writeText(
                                    searchResult[ind][1]
                                );
                            }}
                            className={`mb-4 flex flex-col`}
                        >
                            <span
                                className={`${
                                    selectedResult == ind
                                        ? "underline font-semibold"
                                        : ""
                                } text-xs font-extralight}`}
                            >
                                {result[0]}
                            </span>
                            <span>{result[1]}</span>
                        </p>
                    ))}
            </div>
            <div className="fixed z-20 left-0 bottom-0 w-full items-center justify-center gap-2 flex bg-black px-5 py-3 border-t">
                <div className="relative h-11">
                    <input
                        ref={keyInputRef}
                        className="peer h-full w-full border-b border-blue-gray-200 bg-transparent pt-4 pb-1.5 font-sans text-sm font-normal text-blue-gray-700 outline outline-0 transition-all placeholder-shown:border-blue-gray-200 focus:border-gray-500 focus:outline-0 disabled:border-0 disabled:bg-blue-gray-50"
                        placeholder=" "
                    />
                    <label className="after:content[''] pointer-events-none absolute left-0  -top-1.5 flex h-full w-full select-none !overflow-visible truncate text-[11px] font-normal leading-tight text-gray-500 transition-all after:absolute after:-bottom-1.5 after:block after:w-full after:scale-x-0 after:border-b-2 after:border-gray-500 after:transition-transform after:duration-300 peer-placeholder-shown:text-sm peer-placeholder-shown:leading-[4.25] peer-placeholder-shown:text-blue-gray-500 peer-focus:text-[11px] peer-focus:leading-tight peer-focus:text-gray-400 peer-focus:after:scale-x-100 peer-focus:after:border-gray-400 peer-disabled:text-transparent peer-disabled:peer-placeholder-shown:text-blue-gray-500">
                        Key
                    </label>
                </div>
                |
                <div className="relative h-11">
                    <input
                        ref={valueInputRef}
                        className="peer h-full w-full border-b border-blue-gray-200 bg-transparent pt-4 pb-1.5 font-sans text-sm font-normal text-blue-gray-700 outline outline-0 transition-all placeholder-shown:border-blue-gray-200 focus:border-gray-500 focus:outline-0 disabled:border-0 disabled:bg-blue-gray-50"
                        placeholder=" "
                    />
                    <label className="after:content[''] pointer-events-none absolute left-0  -top-1.5 flex h-full w-full select-none !overflow-visible truncate text-[11px] font-normal leading-tight text-gray-500 transition-all after:absolute after:-bottom-1.5 after:block after:w-full after:scale-x-0 after:border-b-2 after:border-gray-500 after:transition-transform after:duration-300 peer-placeholder-shown:text-sm peer-placeholder-shown:leading-[4.25] peer-placeholder-shown:text-blue-gray-500 peer-focus:text-[11px] peer-focus:leading-tight peer-focus:text-gray-400 peer-focus:after:scale-x-100 peer-focus:after:border-gray-400 peer-disabled:text-transparent peer-disabled:peer-placeholder-shown:text-blue-gray-500">
                        Value
                    </label>
                </div>
                <button
                    onClick={addRecord}
                    className="w-[30px] text-xs font-extralight"
                >
                    Add
                </button>
            </div>
        </div>
    );
}
