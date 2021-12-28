(ns day21.core
  (:require [clojure.math.combinatorics :as combo]))

(def dice-roll (map #(mod % 100) (iterate inc 1)))

(defn player1-seq
  [start-pos]
  (reductions +
              (rest
                (map #(let [m (mod % 10)] (if (= 0 m) 10 m))
                     (reductions +
                                 (conj
                                   (take-nth 2              ;;drop 1 for player 2
                                             (map #(mod (apply + %) 10)
                                                  (partition 3 dice-roll)))
                                   start-pos))))))

;(let [round (inc (count (take-while #(< % 1000) (player2-seq 2))))]
;  (* 6 round (last (take round (player1-seq 1)))))

(def p1-after-round-1 (frequencies (map #(let [x (+ 4 (apply + %))] (if (> x 10) (- x 10) x)) (combo/permuted-combinations '(1 1 1 2 2 2 3 3 3) 3))))
(def p2-after-round-1 (frequencies (map #(+ 2 (mod (apply + %) 10)) (combo/permuted-combinations '(1 1 1 2 2 2 3 3 3) 3))))
(def dice-roll-p2 (frequencies (map #(mod (apply + %) 10) (combo/permuted-combinations '(1 1 1 2 2 2 3 3 3) 3))))

(defn combine
  [m1 m2 acc]
  (if (empty? m1)
    acc
    (let [res (into (sorted-map) (map #(clojure.lang.MapEntry/create (+ (key (first m1)) (key %)) (* (val (first m1)) (val %))) m2))]
      (combine (rest m1) m2 (merge-with + acc res)))))

(defn wins
  [m]
  (apply + (map val (filter #(> (key %) 20) m))))

(defn loses
  [m]
  (apply + (map val (filter #(< (key %) 21) m))))