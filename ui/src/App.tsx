import React, { useEffect, useState, useRef, useCallback, FC } from "react";
import { InitOutput, Sort } from "sorting-visualiser";

import MergeSort, { initialColor } from "./MergeSort";

import "./App.css";

const App: FC<{ wasm: InitOutput }> = ({ wasm }) => {
  const [sorted, setSorted] = useState<Array<number>>([]);
  const [ogUnsorted, setOgUnsorted] = useState<
    Array<{ h: number; color: string }>
  >([]);
  const [unsorted, setUnsorted] = useState<Array<{ h: number; color: string }>>(
    []
  );
  const [animations, setAnimations] = useState<Uint32Array>();
  const [arraySize, setArraySize] = useState(40);
  const staaaphItt = useRef(false);
  const inProgress = useRef(false);

  const generateNewArray = useCallback(() => {
    staaaphItt.current = true;
    inProgress.current = false;
    const memory = wasm.memory;

    // const sort = Sort.new(arraySize);
    const sort = Sort.new_quick_sort(arraySize);
    const animationsLength = sort.animations_length();

    const unsortedPtr = sort.get_unsorted();
    const animationsPtr = sort.get_merge_sort_animations();

    const unsorted = new Uint32Array(memory.buffer, unsortedPtr, arraySize);
    const animations = new Uint32Array(
      memory.buffer,
      animationsPtr,
      animationsLength
    );

    const properArr = Array.from(unsorted as unknown as Array<number>).map(
      (el: number) => {
        return { h: el, color: initialColor };
      }
    );

    setUnsorted(properArr);
    setOgUnsorted(properArr);
    // what the fuck man
    // if i dont do Array<T>, calling map with an HTMLElement returns 0???
    // setUnsorted(unsorted) does not work
    // setSorted(Array.from(sorted));
    setAnimations(animations);
  }, [arraySize]);

  useEffect(() => {
    generateNewArray();
  }, [arraySize, generateNewArray]);

  // return null;

  if (animations)
    return (
      <MergeSort
        generateNewArray={generateNewArray}
        unsorted={unsorted}
        ogUnsorted={ogUnsorted}
        setUnsorted={setUnsorted}
        animations={animations}
        arraySize={arraySize}
        setArraySize={setArraySize}
        staaaphItt={staaaphItt}
        inProgress={inProgress}
      />
    );
  return null;
};

export default App;
