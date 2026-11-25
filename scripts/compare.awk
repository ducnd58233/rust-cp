{
    sub(/[[:space:]]+$/, "")
}

FILENAME == ARGV[1] {
    expected[++n] = $0
    next
}

{
    actual[++m] = $0
}

END {
    print "========== TEST RESULT =========="

    total = n > m ? n : m

    for (i = 1; i <= total; i++) {
        if (i <= n && i <= m && ("s:" expected[i]) == ("s:" actual[i]))
            continue

        errors++

        printf "\n[FAIL] Line %d\n", i
        printf "  Expected : %s\n", (i <= n ? expected[i] : "<missing line>")
        printf "  Actual   : %s\n", (i <= m ? actual[i] : "<missing line>")
    }

    print "\n================================="
    printf "Matched : %d/%d lines\n", total - errors, total
    print "Verdict : " (errors ? "WRONG ANSWER" : "ACCEPTED")

    exit (errors ? 1 : 0)
}