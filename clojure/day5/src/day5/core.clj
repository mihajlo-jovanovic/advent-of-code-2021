(ns day5.core
  (:require [clojure.java.io :as io]
            [clojure.string :as str]))

(defn get-input [file]
  (->> file io/resource slurp))

(defn parse [input]
  (->> input
       (str/split-lines)
       (map #(str/split % #" -> "))
       (map #(let [l (str/split (first %) #",")
                   r (str/split (last %) #",")
                   x1 (Integer/parseInt (first l))
                   y1 (Integer/parseInt (last l))
                   x2 (Integer/parseInt (first r))
                   y2 (Integer/parseInt (last r))]
               {:x1 x1 :y1 y1 :x2 x2 :y2 y2}))))

(defn is-vertical? [l]
  (= (:x1 l) (:x2 l)))

(defn is-horizontal? [l]
  (= (:y1 l) (:y2 l)))