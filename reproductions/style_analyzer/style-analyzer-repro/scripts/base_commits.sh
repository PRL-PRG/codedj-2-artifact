for repo in `</home/anonymous_user/Workspace/style-analyzer/lookout/style/format/benchmarks/data/quality_report_repos.csv tail -n +2 | cut -f 1 -d,`
do 
    git clone ${repo}.git
done

function get_n {
    git log $2 --pretty="format:%H %s" | tac | cat -n | grep $1 | tr -s ' ' | tr '\t' ' ' | cut -f 2 -d ' '
}

function get {
    git log $2 --pretty="format:%H %s %P" | grep "^$1"
}

echo "repo, base@main, head@main, #main, base@all, head@all, #all"
for repo in `</home/anonymous_user/Workspace/style-analyzer/lookout/style/format/benchmarks/data/quality_report_repos.csv tail -n +2`
do 
    base_commit=`echo $repo | cut -f 3 -d,`
    head_commit=`echo $repo | cut -f 2 -d ,`
    repo=`echo $repo | cut -f 1 -d ,`
    dir=`basename $repo .git`
    
    cd $dir
    
    main_count=`git log --pretty="format:%H %s" | wc -l`
    all_count=`git log --all --pretty="format:%H %s" | wc -l`
    
    base_in_main=`get_n $base_commit`
    base_in_all=`get_n $base_commit --all`

    head_in_main=`get_n $head_commit`
    head_in_all=`get_n $head_commit --all`

    #get $base_commit
    #get $head_commit
    
    echo "$dir, $base_in_main, $head_in_main, $main_count, $base_in_all, $head_in_all, $main_count"
    
    cd ..
done

repo, base@main, head@main, #main, base@all, head@all, #all
telescope, 1219, 1219, 1691, 1260, 1260, 1691
carlo, 126, 126, 159, 128, 128, 159
reveal.js, 2076, 2076, 2800, 2096, 2096, 2800
create-react-app, 875, 875, 2716, 971, 971, 2716
axios, 287, 287, 1011, 311, 311, 1011
redux, 812, 812, 3362, 859, 859, 3362
citgm, 365, 365, 847, 369, 369, 847
evergreen, 516, 516, 769, 1117, 1117, 769
30-seconds-of-code, , , 5383, 1680, 1680, 5383
express, 217, 217, 5613, 253, 253, 5613
freeCodeCamp, 10386, 10386, 28459, 10398, 10398, 28459
storybook, 21512, 21512, 35563, 24481, 24481, 35563
jquery, 287, 287, 6534, 409, 409, 6534
core-js, 1359, 1359, 3412, 1556, 1556, 3412
atom, 2699, 2699, 38349, 3344, 3344, 38349
react-native, 8220, 8220, 23050, 10196, 10196, 23050
react, 4039, 4039, 14391, 4236, 4236, 14391
meteor, 2630, 2630, 24177, 3453, 3453, 24177
webpack, 6675, 6675, 14229, 6814, 6814, 14229
node, 9972, 9972, 34364, 24689, 24689, 34364