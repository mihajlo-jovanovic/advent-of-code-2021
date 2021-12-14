(ns day13.core
  (:require [clojure.string :as str]))

(defn move-dot
  [fold-left x y n]
  (if fold-left
    (if (> x n)
      [(- n (- x n)) y]
      [x y])
    (if (> y n)
      [x (- n (- y n))]
      [x y])))

(def fold-left (partial move-dot true))
(def fold-up (partial move-dot false))

(defn fold-along-x
  [coords n]
  (into (hash-set) (map #(fold-left (first %) (second %) n) coords)))

(defn fold-along-y
  [coords n]
  (into (hash-set) (map #(fold-up (first %) (second %) n) coords)))

(defn part2
  [coords]
  (-> coords
      (fold-along-x 655)
      (fold-along-y 447)
      (fold-along-x 327)
      (fold-along-y 223)
      (fold-along-x 163)
      (fold-along-y 111)
      (fold-along-x 81)
      (fold-along-y 55)
      (fold-along-x 40)
      (fold-along-y 27)
      (fold-along-y 13)
      (fold-along-y 6)))

(defn print-sheet
  [coords]
  (let [ncols (apply max (map #(first %) coords))
        nrows (apply max (map #(last %) coords))]
    (reduce #(str %1 "\n" %2)
            (map (fn [r]
                   (reduce str
                           (mapcat
                             #(if (contains? coords (list % r))
                                "#"
                                ".")
                             (range 0 (inc ncols)))))
                 (range 0 (inc nrows))))))

;(def sample #{'(6 10), '(0 14), '(9 10), '(0 3), '(10 4), '(4 11), '(6 0), '(6 12), '(4 1), '(0 13), '(10 12), '(3 4), '(3 0), '(8 4), '(1 10), '(2 14), '(8 10), '(9 0)})
;(printf "%s" (print-sheet (fold-along-x (fold-along-y sample 7) 5)))