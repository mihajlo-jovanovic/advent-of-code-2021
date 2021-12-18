(ns day14.core)

(defn insert
  [p c]
  (let [[f s] p]
    (list f c)))

(defn apply-rules-once
  [r s]
  (apply str (flatten (conj (vec (map #(insert % (get r (apply str %))) (partition 2 1 s))) (last s)))))

(def rules (hash-map "CH" "B",
                     "HH" "N",
                     "CB" "H",
                     "NH" "C",
                     "HB" "C",
                     "HC" "B",
                     "HN" "C",
                     "NN" "C",
                     "BH" "H",
                     "NC" "B",
                     "NB" "B",
                     "BN" "B",
                     "BB" "N",
                     "BC" "B",
                     "CC" "N",
                     "CN" "C"))
(def p1 (partial apply-rules-once rules))
(frequencies (last (take 11 (iterate p1 input))))