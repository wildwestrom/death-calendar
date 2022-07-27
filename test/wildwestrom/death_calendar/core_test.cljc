;; Death Calendar: See how many days you have left to live at a glance.
;; Copyright © 2021 Christian Westrom

;; This program is free software: you can redistribute it and/or modify
;; it under the terms of the GNU Affero General Public License as
;; published by the Free Software Foundation, either version 3 of the
;; License, or (at your option) any later version.

;; This program is distributed in the hope that it will be useful,
;; but WITHOUT ANY WARRANTY; without even the implied warranty of
;; MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
;; GNU Affero General Public License for more details.

;; You should have received a copy of the GNU Affero General Public License
;; along with this program.  If not, see <https://www.gnu.org/licenses/>.

(ns wildwestrom.death-calendar.core-test
  (:require [cljc.java-time.local-date :as local-date]
            [cljc.java-time.period :as period]
            [cljc.java-time.temporal.chrono-unit :as chrono-unit]
            [clojure.test :refer [are deftest is testing]]
            [clojure.test.check.clojure-test :refer [defspec]]
            [clojure.test.check.generators :as gen]
            [clojure.test.check.properties :as prop]
            [tick.core :as t]
            [wildwestrom.death-calendar.core :as sut]))

(def life-expectancy (t/new-period 100 :years))

(deftest death-day
  (testing "Given a birthday and lifespan return death-day."
    (are [death-day b-day lifespan] (= death-day (sut/death-day b-day lifespan))
      (t/new-date 2080 1 1)  (t/new-date 2000 1 1)  (t/new-period 80 :years)
      (t/new-date 2098 8 15) (t/new-date 1998 8 15) (t/new-period 100 :years)
      (t/new-date 2000 2 1)  (t/new-date 2000 1 1)  (t/new-period 1 :months)
      (t/new-date 2001 1 1)  (t/new-date 2000 1 1)  (t/new-period 366 :days)
      (t/new-date 2000 2 29) (t/new-date 2000 1 1)  (t/new-period (+ 30 29) :days)
      (t/new-date 2095 2 28) (t/new-date 1996 2 29) (t/new-period 99 :years))))

(def recent-date-generator
  "Generates a date from 1900-01-01 to a lifetime from now."
  (gen/fmap #(t/new-date %)
            (gen/choose -25567
                        (+ (t/days life-expectancy)
                           (t/today)))))

(def alive-date-generator
  "Generates a birthday such that a person with that birthday
  is not older than a given life expectancy."
  (gen/fmap #(local-date/plus-years (t/today) %)
            (gen/choose
             (+ 1 (- life-expectancy-years))
             life-expectancy-years)))

(def dead-date-generator
  "Generates a birthday such that a person with that birthday
  is older than a given life expectancy."
  (gen/fmap #(local-date/plus-years (t/today) %)
            (gen/choose
             (- (* 2 life-expectancy-years))
             (- (- life-expectancy-years) 1))))

(defspec given-an-alive-date-return-true
  (prop/for-all [bday alive-date-generator]
                (true? (sut/is-alive
                        bday
                        (period/of-years life-expectancy-years)))))

(defspec given-a-dead-date-dead-return-false
  (prop/for-all [bday dead-date-generator]
                (false? (sut/is-alive
                         bday
                         (period/of-years life-expectancy-years)))))
