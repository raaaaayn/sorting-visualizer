import { FC } from "react";
import React, { useState, useRef } from "react";
// import init, { get_merge_sort_animations } from "sorting-visualiser";
import update from "immutability-helper";

import "./MergeSort.css";

function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

const compareColor = "#F8D210";
const ok = "#1bff00";
const notOk = "#F51720";
// const finalColor = "#9b7acb";
const finalColor = "rgba(169, 92, 232, 0.8)";
// const independence = "#545775ff";
export const initalColor = "rgba(66, 134, 244, 0.8)";

const MergeSort: FC<{
  unsorted: Array<{
    h: number;
    color: string;
  }>;
  ogUnsorted: Array<{
    h: number;
    color: string;
  }>;
  generateNewArray: () => void;
  animations: Array<Array<number>>;
  arraySize: number;
  setArraySize: React.Dispatch<number>;
  setUnsorted: React.Dispatch<
    React.SetStateAction<
      {
        h: number;
        color: string;
      }[]
    >
  >;
  staaaphItt: React.MutableRefObject<boolean>;
  inProgress: React.MutableRefObject<boolean>;
}> = ({
  unsorted,
  ogUnsorted,
  arraySize,
  animations,
  setUnsorted,
  setArraySize,
  staaaphItt,
  generateNewArray,
  inProgress,
}) => {
  console.log("state changed");

  const animationSpeed = useRef(50);

  const [animationSpeedDisplay, setAnimationSpeedDisplay] = useState(
    300 - animationSpeed.current
  );

  const mergeSort = async () => {
    if (animations.length > 0) {
      for (let i = 0; i < animations.length - 2; i = i + 3) {
        if (!staaaphItt.current) {
          inProgress.current = true;
          const [idx1, idx2, toExchange] = [
            animations[i],
            animations[i + 1],
            animations[i + 2],
          ];
          // highlights the current elements that are being compared
          setUnsorted((currentUnsorted) => {
            let { h: h1 } = currentUnsorted[idx1];
            let { h: h2 } = currentUnsorted[idx2];
            let updated = update(currentUnsorted, {
              [idx1]: { $set: { h: h1, color: compareColor } },
              [idx2]: { $set: { h: h2, color: compareColor } },
            });
            return updated;
          });
          if (toExchange) {
            await sleep(animationSpeed.current);
            // marks the current elements that are to be exchanged
            setUnsorted((currentUnsorted) => {
              const { h: h1 } = currentUnsorted[idx1];
              const { h: h2 } = currentUnsorted[idx2];
              const color = notOk;
              let updated = update(currentUnsorted, {
                [idx1]: { $set: { h: h1, color } },
                [idx2]: { $set: { h: h2, color } },
              });
              return updated;
            });
            await sleep(animationSpeed.current / 4);
            // shifting the elements in the array and marking the elements that have been exhanged
            setUnsorted((currentUnsorted) => {
              let first = currentUnsorted[idx1];
              let second = currentUnsorted[idx2];
              const color = initalColor;
              let updated = update(currentUnsorted, {
                $splice: [[idx2, 1]],
              });
              updated.splice(idx1, 0, second);
              first = updated[idx1];
              second = updated[idx1 + 1];
              updated[idx1] = { ...first, color };
              updated[idx1 + 1] = { ...second, color };
              return updated;
            });
          } else {
            await sleep(animationSpeed.current);
            setUnsorted((currentUnsorted) => {
              const { h: h1 } = currentUnsorted[idx1];
              const { h: h2 } = currentUnsorted[idx2];
              const color = ok;
              // no need to exchange
              let updated = update(currentUnsorted, {
                [idx1]: { $set: { h: h1, color } },
                [idx2]: { $set: { h: h2, color } },
              });
              return updated;
            });
            await sleep(animationSpeed.current / 4);
            setUnsorted((currentUnsorted) => {
              const { h: h1 } = currentUnsorted[idx1];
              const { h: h2 } = currentUnsorted[idx2];
              const color = initalColor;
              // no need to exchange
              let updated = update(currentUnsorted, {
                [idx1]: { $set: { h: h1, color } },
                [idx2]: { $set: { h: h2, color } },
              });
              return updated;
            });
          }
        }
        inProgress.current = false;
      }
      await sleep(100);
      setUnsorted((currentUnsorted) => {
        return currentUnsorted.map((el) => {
          return { ...el, color: "#32CD30" };
        });
      });
      await sleep(500);
      setUnsorted((currentUnsorted) => {
        return currentUnsorted.map((el) => {
          return { ...el, color: finalColor };
        });
      });
    }
  };

  return (
    <>
      <nav className="nav-bar">
        <div className="input-container">
          <div className="range-input">
            <label htmlFor="changeArrSize">Array size</label>
            <input
              id="changeArrSize"
              type="range"
              min="2"
              max="300"
              value={arraySize}
              onChange={(e) => {
                setArraySize(parseInt(e.target.value));
              }}
            />
            <span>{unsorted.length}</span>
          </div>
          <div className="range-input">
            <label htmlFor="changeSize">Speed</label>
            <input
              id="changeSize"
              type="range"
              min="0"
              max="300"
              value={animationSpeedDisplay}
              onChange={(e) => {
                animationSpeed.current = 300 - parseInt(e.target.value);
                setAnimationSpeedDisplay(parseInt(e.target.value));
              }}
            />
            <span>{animationSpeedDisplay}</span>
          </div>
        </div>

        <button
          onClick={() => {
            staaaphItt.current = true;
            inProgress.current = false;
            generateNewArray();
          }}
        >
          Generate new array
        </button>

        {!inProgress.current && (
          <button
            onClick={() => {
              if (!inProgress.current) {
                console.log(inProgress.current);
                staaaphItt.current = false;
                setUnsorted(ogUnsorted);
                mergeSort();
              }
            }}
          >
            Merge sort!!!
          </button>
        )}

        {inProgress.current && (
          <button
            onClick={() => {
              staaaphItt.current = true;
              inProgress.current = false;
            }}
          >
            Staaphh Itt!
          </button>
        )}
      </nav>
      <main className="merge-sort">
        <ul className="array-container">
          {unsorted.map(({ h, color }, i) => (
            <li
              key={i + h + Math.random()}
              value={h}
              className="array-element"
              style={{
                height: `${h}px`,
                backgroundColor: color,
                maxWidth: `${1000 / unsorted.length}px`,
                border: arraySize > 105 ? "none" : "1px solid black",
                borderTop: "none",
              }}
            >
              {1000 / unsorted.length > 30 ? h : null}
            </li>
          ))}
        </ul>
      </main>
    </>
  );
};

export default MergeSort;
