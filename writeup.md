HOW TO RUN:  cargo run -- --period weekly --smooth 3 data/AirQualityUCI.csv
The ‘weekly” can be changed with daily and monthly
OUTPUT: 
2004-W11, 2.39, 105.70, 1111.47
2004-W12, 2.50, 106.69, 1165.57
2004-W13, 2.29, 100.24, 1064.38
2004-W14, 2.03, 89.30, 1013.55
2004-W15, 1.80, 81.90, 858.81
2004-W16, 1.66, 71.69, 890.01
2004-W17, 1.41, 59.58, 916.74
2004-W18, 1.52, 61.07, 1014.60
2004-W19, 1.53, 62.71, 982.81
2004-W20, 1.92, 80.81, 983.16
2004-W21, 1.83, 82.22, 966.41
2004-W22, 1.56, 78.22, 931.74
2004-W23, 1.18, 65.13, 873.85
2004-W24, 1.18, 65.73, 863.57
2004-W25, 1.55, 78.29, 845.03
2004-W26, 1.86, 91.56, 855.99
2004-W27, 1.83, 93.87, 874.82
2004-W28, 1.85, 99.95, 989.31
2004-W29, 1.86, 103.27, 1015.45
2004-W30, 1.59, 101.55, 1039.86
2004-W31, 1.11, 96.53, 973.71
2004-W32, 0.74, 84.63, 932.43
2004-W33, 0.75, 70.69, 812.49
2004-W34, 0.78, 47.61, 767.24
2004-W35, 0.87, 43.42, 668.01
2004-W36, 0.87, 32.86, 746.34
2004-W37, 1.11, 40.61, 806.78
2004-W38, 1.23, 44.70, 963.13
2004-W39, 1.61, 68.81, 974.70
2004-W40, 1.75, 77.51, 1071.15
2004-W41, 1.68, 67.71, 1143.22
2004-W42, 1.07, 40.10, 1096.94
2004-W43, 1.12, 34.62, 1152.36
2004-W44, 1.64, 53.41, 1140.38
2004-W45, 2.34, 76.84, 1258.39
2004-W46, 2.31, 87.39, 1094.45
2004-W47, 2.22, 104.67, 1065.09
2004-W48, 2.52, 121.75, 1158.76
2004-W49, 2.90, 128.65, 1253.83
2004-W50, 2.76, 114.55, 1223.35
2004-W51, 2.66, 114.64, 948.57
2004-W52, 2.74, 123.11, 965.12
2004-W53, 2.33, 105.91, 920.24
2005-W01, 2.07, 103.32, 974.34
2005-W02, 1.91, 102.58, 1001.69
2005-W03, 2.26, 134.71, 1132.63
2005-W04, 1.76, 127.69, 1041.53
2005-W05, 1.66, 138.42, 1021.72
2005-W06, 1.87, 145.31, 823.94
2005-W07, 2.15, 156.86, 948.88
2005-W08, 2.08, 154.09, 851.66
2005-W09, 1.81, 141.78, 912.21
2005-W10, 1.81, 142.86, 952.78
2005-W11, 1.89, 141.01, 1032.02
2005-W12, 2.08, 139.11, 1147.28
2005-W13, 2.00, 127.49, 1055.01
2005-W14, 1.89, 118.80, 1000.58

Throughout my life I have been invested in global warming. So I found this dataset in order to see how CO, NO2 and O3 levels in an urban environment flow over time. I used the UCI Air quality dataset readings from March 2004 to April 2005. In parser.rs I defined a RawRecord to mirror csv header and an AirQualityENtry in order to hold the cleaned data. I replaced every comma with a dot so the 2,6 became 2.6, parsed it as f32 and filtered out the UCI’s -200 values by dropping anything below zero. For timestamps I concatenated the Data and Time strings into “dd/mm/YYYY HH.MM.SS, parsed that into the NaiveDataTime, and wrapped it FixedOffset, allowing me to skip any empty or ruined rows. Doing so gave me the Vec<AirQualityENtry>. 

Then in utils.rs I wrote filter_outliers to keep only entries where CO sat in 0-15 mg/m^3 and NO2 in 0-200 in order to avoid any outliers that would have been caused by wrong sensor readings. For which then I created a smooth_stats function that when given a window size it computes a sliding window average so that each of the period’s stats become the mean of that period and the two preceding ones. 

In analysis.rs  I factored out aggregate_by which groups entries (grouping by the e.datetime.format(“%Y-%m”). Folding sums into counts to produce MonthlyStats structs of co_avg, no2_avg, and o3_avg. 

Then finally report.rs simply prints a CSV header “Period, CO_avg, no2_avg, o3_avg”, and then each sorter two with two decimal places. 

My main.rs ties this all together with clap, where you can run a project –period weekly –smooth 3 data/AirQualityUCI.csv and it will then parse, clean, filter, aggregate and smooth everything in one go. To be sure this worked, I wrote two tests in my project, one was initially for parser tests to confirm that a minimal csv line produced exactly one entry with correct CO value, and one for analysis where it would check that feeding a single entry month_average would result in proper key and average. With the cargo test both of them DID NOT pass. Through various debugging efforts I could not get it to work. 

Results: While I obviously could not confirm my results because of my struggle with the tests, I did see some reasonable results. I saw that the CO clustered around 1.5-2.5 mg/m^3 and NO2 near 100 units and O3 sensor readings around given my time. The ranges are something I expected and am hoping are true again given that my tests were not correctly working. 
