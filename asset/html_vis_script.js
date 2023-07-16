document.addEventListener("DOMContentLoaded", function() {
    let calenders = document.querySelectorAll(".sched-root");
    let activeIndex = 0;

    hideAll(calenders);
    show(calenders[activeIndex]);

    document.addEventListener("keypress", function(e) {
        if (e.key == "j") activeIndex++;
        if (e.key == "k") activeIndex--;

        if(activeIndex < 0) activeIndex = 0;
        if(activeIndex >= calenders.length) activeIndex = calenders.length - 1;

        hideAll(calenders);
        show(calenders[activeIndex]);
    })
});

function show(cal) {
    cal.style.display = "block";
}

function hideAll(cals) {
    cals.forEach(x=>x.style.display = "none");
}