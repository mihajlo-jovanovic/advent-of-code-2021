(ns day7.core-test
  (:require [clojure.test :refer :all]
            [day7.core :refer :all]))

(deftest crab-movement
  (testing "crab movement"
    (let [crabs (parse "16,1,2,0,4,2,7,1,2,14")]
      (is (= 37 (fuel 2 crabs)))
      (is (= 41 (fuel 1 crabs)))
      (is (= 39 (fuel 3 crabs)))
      (is (= 71 (fuel 10 crabs)))
      (is (= 168 (fuel-p2 5 crabs)))
      (is (= 206 (fuel-p2 2 crabs)))
      (is (= 37 (solution crabs fuel)))
      (is (= 168 (solution crabs fuel-p2))))))
