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

(ns wildwestrom.death-calendar.core
  (:import (java.time LocalDate)
           (java.time.temporal TemporalAmount ChronoUnit))
  (:gen-class))

(set! *warn-on-reflection* true)

(defn death-day
  [^LocalDate birth-day ^TemporalAmount life-span]
  (.plus birth-day life-span))

(defn total-days
  [^LocalDate birth-day ^TemporalAmount life-span ^ChronoUnit unit]
  (.between unit
            birth-day
            (death-day birth-day life-span)))

(defn lived-days
  [^LocalDate birth-day ^TemporalAmount life-span ^ChronoUnit unit]
  (let [calculated (.between unit
                             (LocalDate/now)
                             (death-day birth-day life-span))]
    calculated))

(defn calendar-data
  [^LocalDate birth-day
   ^TemporalAmount life-span
   & {:keys [unit]
      :or {unit ChronoUnit/DAYS}}]
  (let [total-life (total-days birth-day life-span unit)
        remaining  (lived-days birth-day life-span unit)
        lived      (- total-life remaining)
        cal-map    {:total     total-life
                    :lived     lived
                    :remaining remaining}]
    (if (> 0 remaining)
      (assoc cal-map :dead? true)
      cal-map)))
