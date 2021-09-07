# Model report for file:///tmp/top-repos-quality-repos-i3dkitfq/urule.git HEAD 1b7edeef8bb6e9b365b249cb9900fe82cf23ad5d

### Dump

```json
{'created_at': '2021-09-02 05:48:42',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-74-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '22.6 kB',
 'tags': [],
 'uuid': 'be79b8bc-5aa3-4016-806c-23f382ade53a',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-i3dkitfq/urule.git 1b7edeef8bb6e9b365b249cb9900fe82cf23ad5d

# javascript
50 rules, avg.len. 8.4
## train
PPCR: 0.898633
### report
macro
{'f1-score': 0.6609443936321527,
 'precision': 0.7424508878754587,
 'recall': 0.6201253033457991,
 'support': 89325}
micro
{'f1-score': 0.9560033585222502,
 'precision': 0.9560033585222502,
 'recall': 0.9560033585222502,
 'support': 89325}
weighted
{'f1-score': 0.9519495130150861,
 'precision': 0.9545993845517233,
 'recall': 0.9560033585222502,
 'support': 89325}
### report_full
macro
{'f1-score': 0.49733145034239007,
 'precision': 0.7424508878754587,
 'recall': 0.4117986019811016,
 'support': 99401}
micro
{'f1-score': 0.904962750230493,
 'precision': 0.9560033585222502,
 'recall': 0.8590959849498496,
 'support': 99401}
weighted
{'f1-score': 0.8860521632834646,
 'precision': 0.9482450911680328,
 'recall': 0.8590959849498496,
 'support': 99401}
## test
PPCR: 0.898275
### report
macro
{'f1-score': 0.6004987922083594,
 'precision': 0.7015542800935778,
 'recall': 0.5568752526442596,
 'support': 24416}
micro
{'f1-score': 0.9540465268676278,
 'precision': 0.9540465268676278,
 'recall': 0.9540465268676278,
 'support': 24416}
weighted
{'f1-score': 0.9492302326669043,
 'precision': 0.9528356209080467,
 'recall': 0.9540465268676278,
 'support': 24416}
### report_full
macro
{'f1-score': 0.4569175673981185,
 'precision': 0.7015542800935778,
 'recall': 0.38402588909113883,
 'support': 27181}
micro
{'f1-score': 0.9029207124445219,
 'precision': 0.9540465268676278,
 'recall': 0.8569956955226077,
 'support': 27181}
weighted
{'f1-score': 0.8808037343452487,
 'precision': 0.9478481650834847,
 'recall': 0.8569956955226077,
 'support': 27181}
```

## javascript
### Summary
30 rules, avg.len. 8.4

| | |
|-|-|
|Min support|91|
|Max support|47202|
|Min confidence|0.9203869104385376|
|Max confidence|0.9994742274284363|

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
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 190,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  •••start_col ≥ 11<br>	∧ -1.reserved = ;<br>	∧ +1.reserved not in {}}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ⏎<br>Confidence: 0.930. Support: 3157.` |
| 2 | `  -1.reserved not in {;}<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.976. Support: 231.` |
| 3 | `  -1.reserved not in {;}<br>	∧ -2.label in {<newline>} and not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.952. Support: 155.` |
| 4 | `  -1.reserved not in {;}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +3.length ≥ 6<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.941. Support: 195.` |
| 5 | `  •••start_col ≤ 19<br>	∧ -1.reserved not in {;}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -3.diff_offset ≤ 14<br>	∧ -3.reserved not in {{}<br>	∧ -5.diff_offset ≤ 45<br>	∧ -5.reserved not in {:}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = "<br>Confidence: 0.920. Support: 672.` |
| 6 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;}<br>	∧ -4.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {Program}<br>⇒ y = '<br>Confidence: 0.984. Support: 91.` |
| 7 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;}<br>	∧ -4.reserved not in {{}<br>	∧ -4.length ≤ 7<br>	∧ -5.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {Program}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = "<br>Confidence: 0.923. Support: 1391.` |
| 8 | `  •••start_col ≤ 31<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {BINARY}<br>	∧ ^2.roles in {TYPE}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.995. Support: 109.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = var<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 951.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, var, {}<br>	∧ -2.label in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {KEY}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.995. Support: 286.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 427.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 298.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>⇒ y = ∅<br>Confidence: 0.998. Support: 213.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, var, {}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>⇒ y = ␣<br>Confidence: 0.927. Support: 981.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, const, return, var, {}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.927. Support: 3356.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = new<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 197.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = export<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 180.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, var, {}<br>	∧ -2.internal_type = StringLiteral<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 176.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = BlockStatement<br>⇒ y = ⏎<br>Confidence: 0.925. Support: 580.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.980. Support: 1545.` |
| 21 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {;, var, {, }}<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved not in {}}<br>⇒ y = ⏎<br>Confidence: 0.960. Support: 137.` |
| 22 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = let<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 119.` |
| 23 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = import<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 95.` |
| 24 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ +3.reserved = .<br>	∧ ^1.internal_type = LogicalExpression<br>⇒ y = ␣<br>Confidence: 0.975. Support: 100.` |
| 25 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = function<br>	∧ -2.label not in {<space>}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ -4.length ≥ 6<br>	∧ -5.diff_line ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {LogicalExpression}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 98.` |
| 26 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = function<br>	∧ -2.label not in {<space>}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ -4.length ≤ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {LogicalExpression}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 510.` |
| 27 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, function, let, return, var, {, }}<br>	∧ -2.label not in {<space>}<br>	∧ -4.reserved = function<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {if, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {LogicalExpression}<br>	∧ ^1.roles in {FUNCTION}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 140.` |
| 28 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {, }}<br>	∧ -2.label not in {<space>}<br>	∧ -4.reserved = function<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 92.` |
| 29 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, function, let, return, var, {, }}<br>	∧ -2.label not in {<space>}<br>	∧ -2.length ≤ 4<br>	∧ -4.reserved not in {function}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {if, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {LogicalExpression}<br>	∧ ^2.internal_type = Program<br>⇒ y = ∅<br>Confidence: 0.970. Support: 148.` |
| 30 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, function, let, return, var, {, }}<br>	∧ -2.label not in {<space>}<br>	∧ -4.reserved not in {function}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {if, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {LogicalExpression}<br>	∧ ^2.internal_type not in {Program}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 47202.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.366666666666667, "max_conf": 0.9994742274284363, "max_support": 47202, "min_conf": 0.9203869104385376, "min_support": 91, "num_rules": 30}}
```
</details>
