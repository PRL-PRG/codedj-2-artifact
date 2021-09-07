# Model report for file:///tmp/top-repos-quality-repos-uw4ywgtf/create-react-app.git HEAD 08dc7ab0f520f680b678556edeb702a5791af20e

### Dump

```json
{'created_at': '2021-09-02 01:02:03',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.19.0-12-amd64-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '17.4 kB',
 'tags': [],
 'uuid': '161ce456-3c45-4a2c-a747-b5451e40bb3a',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-uw4ywgtf/create-react-app.git 08dc7ab0f520f680b678556edeb702a5791af20e

# javascript
73 rules, avg.len. 10.0
## train
PPCR: 0.937932
### report
macro
{'f1-score': 0.821950892991807,
 'precision': 0.9004477087448539,
 'recall': 0.7685989588773434,
 'support': 21156}
micro
{'f1-score': 0.9031953110228778,
 'precision': 0.9031953110228776,
 'recall': 0.9031953110228776,
 'support': 21156}
weighted
{'f1-score': 0.8997179424508362,
 'precision': 0.9042544056687375,
 'recall': 0.9031953110228776,
 'support': 21156}
### report_full
macro
{'f1-score': 0.758896097433371,
 'precision': 0.9004477087448539,
 'recall': 0.6829603733884608,
 'support': 22556}
micro
{'f1-score': 0.8742679355783309,
 'precision': 0.9031953110228776,
 'recall': 0.8471360170242951,
 'support': 22556}
weighted
{'f1-score': 0.863934005761169,
 'precision': 0.9046995539731567,
 'recall': 0.8471360170242951,
 'support': 22556}
## test
PPCR: 0.887309
### report
macro
{'f1-score': 0.8399319460475672,
 'precision': 0.9278496833211136,
 'recall': 0.8202883218835363,
 'support': 811}
micro
{'f1-score': 0.9247842170160296,
 'precision': 0.9247842170160296,
 'recall': 0.9247842170160296,
 'support': 811}
weighted
{'f1-score': 0.9205543547840916,
 'precision': 0.9277240965238558,
 'recall': 0.9247842170160296,
 'support': 811}
### report_full
macro
{'f1-score': 0.7549710315916752,
 'precision': 0.9278496833211136,
 'recall': 0.6993607384577738,
 'support': 914}
micro
{'f1-score': 0.8695652173913044,
 'precision': 0.9247842170160296,
 'recall': 0.8205689277899344,
 'support': 914}
weighted
{'f1-score': 0.8472257390344847,
 'precision': 0.919921086483862,
 'recall': 0.8205689277899344,
 'support': 914}
```

## javascript
### Summary
34 rules, avg.len. 9.9

| | |
|-|-|
|Min support|152|
|Max support|7258|
|Min confidence|0.9247037768363953|
|Max confidence|0.9996156692504883|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 1.000. Support: 1301.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.967. Support: 352.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.957. Support: 1327.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.998. Support: 220.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.997. Support: 158.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved = :<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^2.roles not in {INITIALIZATION}<br>⇒ y = ⏎<br>Confidence: 0.928. Support: 174.` |
| 7 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, ;, {}<br>⇒ y = ⏎<br>Confidence: 0.974. Support: 436.` |
| 8 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {DECLARATION, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 540.` |
| 9 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.roles in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 424.` |
| 10 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = const<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 187.` |
| 11 | `  -1.diff_col ≤ 1<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, const, {}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.reserved not in {;, }}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), {}<br>	∧ +1.roles in {IDENTIFIER}<br>	∧ +2.reserved not in {;, >}<br>	∧ +5.reserved not in {{}<br>	∧ ^1.roles not in {DECLARATION, IF, MODULE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 947.` |
| 12 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, const, {}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.reserved not in {;, }}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), {}<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ +2.reserved not in {;, >}<br>	∧ +5.reserved not in {{}<br>	∧ ^1.roles not in {DECLARATION, IF, MODULE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 4724.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 891.` |
| 14 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {{}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.982. Support: 463.` |
| 15 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 518.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.998. Support: 241.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.997. Support: 152.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.976. Support: 3533.` |
| 19 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 477.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {KEY}<br>	∧ +3.roles not in {STRING}<br>	∧ ^1.internal_type not in {CallExpression}<br>⇒ y = ⏎<br>Confidence: 0.951. Support: 194.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^2.roles not in {INITIALIZATION}<br>⇒ y = ⏎<br>Confidence: 0.930. Support: 178.` |
| 22 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, ;}<br>⇒ y = ⏎<br>Confidence: 0.974. Support: 440.` |
| 23 | `  -1.diff_col ≤ 1<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.reserved not in {., ;, }}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles in {IDENTIFIER}<br>	∧ +2.reserved not in {;, >}<br>	∧ +4.reserved not in {>}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 1083.` |
| 24 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.reserved not in {., ;, }}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ +2.reserved not in {;, >}<br>	∧ +4.reserved not in {>}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 5321.` |
| 25 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {DECLARATION}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.947. Support: 540.` |
| 26 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, const, {}<br>	∧ -1.length ≤ 1<br>	∧ -2.roles not in {MAP}<br>	∧ -3.reserved not in {;, }}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), {}<br>	∧ +1.roles in {IDENTIFIER}<br>	∧ +2.reserved not in {;, >}<br>	∧ +4.reserved not in {>}<br>	∧ ^1.roles not in {CONDITION, DECLARATION, OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 993.` |
| 27 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, const, {}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.reserved not in {;, }}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), {}<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ +2.reserved not in {;, >}<br>	∧ +4.reserved not in {>}<br>	∧ ^1.roles not in {CONDITION, DECLARATION, OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 5098.` |
| 28 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.946. Support: 528.` |
| 29 | `  -1.internal_type = CommentLine<br>⇒ y = ⏎<br>Confidence: 0.959. Support: 499.` |
| 30 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.998. Support: 274.` |
| 31 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, const, {}<br>	∧ -2.roles not in {MAP, STRING}<br>	∧ -3.reserved not in {;, }}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), {}<br>	∧ +2.reserved not in {;, >}<br>	∧ +5.reserved not in {{}<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 3579.` |
| 32 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, const, {}<br>	∧ -2.roles not in {MAP, STRING}<br>	∧ -3.reserved not in {;, }}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), {}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {;, >}<br>	∧ +5.reserved not in {{}<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 475.` |
| 33 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, const, {}<br>	∧ -2.roles not in {MAP, STRING}<br>	∧ -3.reserved not in {;, }}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), {}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {;, >}<br>	∧ +5.reserved not in {{}<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 1663.` |
| 34 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, STATEMENT}<br>	∧ ^2.internal_type not in {File}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 7258.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.882352941176471, "max_conf": 0.9996156692504883, "max_support": 7258, "min_conf": 0.9247037768363953, "min_support": 152, "num_rules": 34}}
```
</details>
