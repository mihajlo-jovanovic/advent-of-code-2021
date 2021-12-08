(ns day7.core
  (:require [clojure.string :as str]
            [clojure.java.io :as io])
  (:gen-class))

(defn parse [s]
  (->> (str/split s #",")
       (map #(Integer/parseInt %))))

(defn fuel
  [x l]
  (apply + (map #(Math/abs (- % x)) l)))

(defn fuel-p2
  [x l]
  (apply +
         (map #(->> (Math/abs (- % x))
                    (inc)
                    (range 1)
                    (apply +)) l)))

(defn solution [l f]
  (apply min (for [x (range (apply min l) (apply max l))] (f x l))))

(defn -main
  [& _]
  (let [input (-> "day7.txt"
                  io/resource
                  str
                  slurp
                  parse)]
    (println "Part 1 solution: " (solution input fuel))
    (println "Part 2 solution: " (solution input fuel-p2))))