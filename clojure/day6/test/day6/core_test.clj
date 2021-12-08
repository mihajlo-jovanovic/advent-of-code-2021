(ns day6.core-test
  (:require [clojure.test :refer :all]
            [day6.core :refer :all]))

(deftest lanternfish-recur-test
  (testing "lanternfish simulation using sample data"
    (let [state (parse "3,4,3,1,2")]
      (is (= 26 (lanternfish-recur state, 18)))
      (is (= 5934 (lanternfish-recur state 80)))
      (is (= 26984457539 (lanternfish-recur state 256))))))
