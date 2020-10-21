library(tidyverse)
ramachandran <- read_csv('ramachandran.csv')
ramachandran %>%
  ggplot(aes(phi, psi)) +
  geom_hline(yintercept = 0) +
  geom_vline(xintercept = 0) +
  geom_point(alpha=0.3)+
  scale_x_continuous(limits = c(-180, 180), breaks = seq(-180, 180, 90), expand = c(0,0))+
  scale_y_continuous(limits = c(-180, 180), breaks = seq(-180, 180, 90), expand = c(0,0))+
  labs(title="Ramachandran Plot 4F7I")+
  theme_bw()+
  theme(plot.title = element_text(hjust = 0.5), plot.margin = margin(8,14,2,5))

ggsave("ramachandran.png")