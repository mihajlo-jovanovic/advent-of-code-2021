(ns day6.core
  (:require [clojure.string :as str]
            [clojure.java.io :as io])
  (:gen-class))

(defn parse [s]
  (->> (str/split s #",")
       (map #(Integer/parseInt %))
       (frequencies)
       (into (sorted-map))))

(defn simulate [m]
  (let [f (first m)
        r (rest m)
        mn (inc (key f))]
    (->> (zipmap (map #(- % mn) (keys r)) (vals r))
         (merge-with + {6 (val f)} {8 (val f)})
         (into (sorted-map)))))

(defn lanternfish-recur
  [m i]
  (if (<= i (key (first m)))
    (reduce + (vals m))
    (lanternfish-recur (simulate m) (- i (key (first m)) 1))))

(defn -main
  [& _]
  (let [input (-> "day6.txt"
                  io/resource
                  str
                  slurp
                  parse)]
    (println "Part 1 solution: " (lanternfish-recur input 80))
    (println "Part 2 solution: " (lanternfish-recur input 256))))