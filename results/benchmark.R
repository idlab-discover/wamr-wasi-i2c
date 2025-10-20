library(ggplot2)
# library(dplyr)

# native <- read.csv("native.csv")
# wamr <- read.csv("wamr.csv")
# wasmtime <- read.csv("wasmtime.csv")
# 
# df <- list(native, wamr, wasmtime)
# df <- Reduce(function(x, y)
#   merge(x, y, all = TRUE), df)

df <- read.csv("merged.csv")

### Filter out the outliers, just for funsies

# list_quantiles <- tapply(data$time, data$run, quantile)
# Q1s <- sapply(1:3, function(i) list_quantiles[[i]][2])
# Q3s <- sapply(1:3, function(i) list_quantiles[[i]][4])
# 
# IQRs <- tapply(data$time, data$run, IQR)
# 
# Lowers <- Q1s - 1.5*IQRs
# Uppers <- Q3s + 1.5*IQRs
# 
# datas <- split(data, data$run)
# 
# data_no_outlier <- NULL
# for (i in 1:3){
#   out <- subset(datas[[i]], datas[[i]]$time > Lowers[i] & datas[[i]]$time < Uppers[i])
#   data_no_outlier <- rbind(data_no_outlier, out)
# }

###
# df <- data_no_outlier
# write.csv(df, "merged.csv")

df$time <- df$time / 1e3

# means <- aggregate(time ~ type +  run, df, mean)
means <- aggregate(time ~ run, df, mean)

ggplot(data = df, aes(x = run, y = time)) +
  geom_boxplot() +
  stat_summary(
    fun = mean,
    colour = "darkred",
    geom = "point",
    shape = 18,
    size = 3,
    show.legend = FALSE,
    position = position_dodge(width = 1)
  ) +
  # geom_text(data = means,
  #           aes(label = paste(round(time, digits = 2), "\u03bc\u0073"), y = time + 35),
  #           position = position_dodge(width = 0.8)) +
  ylab("time (\u03bc\u0073)")
