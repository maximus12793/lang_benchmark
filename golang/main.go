package main

import (
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"runtime"
	"runtime/pprof"
	"sync"
	"time"
)

func grabPage(i int, wg *sync.WaitGroup) {
	defer wg.Done()
	res, err := http.Get("https://en.wikipedia.org/wiki/Immanuel_Kant")
	if err != nil {
		log.Fatal(err)
	}
	f, err := os.Create(fmt.Sprintf("./data/%d.txt", i))
	if err != nil {
		log.Fatal(err)
	}
	_, err = io.Copy(f, res.Body)
	if err != nil {
		log.Fatal(err)
	}
	res.Body.Close()
}

func main() {
	f, _ := os.Create("cpuprofile")

	pprof.StartCPUProfile(f)
	defer pprof.StopCPUProfile()
	runtime.GOMAXPROCS(4)
	start := time.Now()
	var wg sync.WaitGroup
	total := 800

	wg.Add(total)
	for index := 0; index < total; index++ {
		go grabPage(index, &wg)
	}
	wg.Wait()
	elapsed := time.Since(start)
	log.Printf("took %s", elapsed)
}
