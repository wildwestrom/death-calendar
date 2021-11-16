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

(ns wildwestrom.death-calendar.model-test
  (:require [clojure.test :refer [deftest testing is are]]
            [wildwestrom.death-calendar.model :as sut]
            [clojure.test.check :as tc]
            [clojure.test.check.generators :as gen]
            [clojure.test.check.properties :as prop]
            [clojure.test.check.clojure-test :refer [defspec]])
  (:import (java.time LocalDate Period)
           (java.time.temporal ChronoUnit)))

(def ^:const life-expectancy-years 100)

(deftest death-day
  (testing "Given a birthday and lifespan return death-day."
    (are [death-day b-day lifespan] (= death-day (sut/death-day b-day lifespan))
      (LocalDate/of 2080 1 1)  (LocalDate/of 2000 1 1)  (Period/ofYears 80)
      (LocalDate/of 2098 8 15) (LocalDate/of 1998 8 15) (Period/ofYears 100)
      (LocalDate/of 2000 2 1)  (LocalDate/of 2000 1 1)  (Period/ofMonths 1)
      (LocalDate/of 2001 1 1)  (LocalDate/of 2000 1 1)  (Period/ofDays 366)
      (LocalDate/of 2000 2 29) (LocalDate/of 2000 1 1)  (Period/ofDays (+ 30 29)))))

(def date-generator
  (gen/fmap #(LocalDate/ofEpochDay %)
            (gen/choose (+ -25550 100)
                        (+ (* 365 life-expectancy-years)
                           (.toEpochDay (LocalDate/now))))))

(def alive-date-generator
  (gen/fmap #(.plusYears (LocalDate/now) %)
            (gen/choose
             (+ 1 (- life-expectancy-years))
             life-expectancy-years)))

(def dead-date-generator
  (gen/fmap #(.plusYears (LocalDate/now) %)
            (gen/choose
             (- (* 2 life-expectancy-years))
             (- (- life-expectancy-years) 1))))

(defspec given-an-alive-date-there-is-no-dead-field
  (prop/for-all [bday alive-date-generator]
                (-> (sut/calendar-data
                     bday
                     (Period/ofYears life-expectancy-years))
                    :dead?
                    nil?)))

(defspec given-a-dead-date-dead-is-true
  (prop/for-all [bday dead-date-generator]
                (-> (sut/calendar-data
                     bday
                     (Period/ofYears life-expectancy-years))
                    :dead?
                    true?)))

(defspec output-contains-all-required-fields
  (prop/for-all [output (gen/fmap
                         #(sut/calendar-data
                           % (Period/ofYears life-expectancy-years))
                         dead-date-generator)]
                (and (int? (:lived output))
                     (int? (:total output))
                     (int? (:remaining output))
                     (boolean? (:dead? output)))))

(defspec ChronoUnit-days-is-equal-to-no-ChronoUnit-specified
  (prop/for-all [date date-generator
                 num-of-weeks (gen/fmap #(Period/ofWeeks %) (gen/fmap #(* 52 %) (gen/choose -100 100)))]
                (is (= (sut/calendar-data date num-of-weeks)
                       (sut/calendar-data date num-of-weeks :unit ChronoUnit/DAYS)))))
