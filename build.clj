(ns build
  (:refer-clojure :exclude [test])
  (:require [clojure.tools.build.api :as b]
            [org.corfield.build :as bb]))

(def lib 'wildwestrom/death-calendar)
(def version (format "0.0.%s" (b/git-count-revs nil)))

(defn clean [_]
  (b/delete {:path "target"}))

(defn test [opts]
  (bb/run-tests opts))

(defn ci [opts]
  (-> opts
      (assoc :lib lib :version version)
      (bb/run-tests)
      (bb/clean)
      (bb/jar)))
